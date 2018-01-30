use core::fmt::Write;
use cortex_m_semihosting::hio;
use stm32l151::{DMA1, GPIOA, RCC, USART2};
use super::protocol::MsgType;

pub struct Serial<'a> {
    usart: USART2,
    receive_stage: ReceiveStage,
    send_buffer: &'a mut[u8; 0x10],
    receive_buffer: &'a mut[u8; 0x10],
}

pub struct Message<'a> {
    pub msg_type: u8,
    pub operation: u8,
    pub data: &'a[u8],
}

enum ReceiveStage {
    Header,
    Body,
}

impl<'a> Serial<'a> {
    pub fn new(usart: USART2, dma: &DMA1, gpioa: &mut GPIOA, rcc: &mut RCC,
               send_buffer: &'a mut[u8; 0x10], receive_buffer: &'a mut[u8; 0x10]) -> Serial<'a> {
        let mut serial = Serial {
            usart: usart,
            receive_stage: ReceiveStage::Header,
            send_buffer: send_buffer,
            receive_buffer: receive_buffer,
        };
        serial.init(dma, gpioa, rcc);
        serial
    }

    fn init(&mut self, dma: &DMA1, gpioa: &mut GPIOA, rcc: &mut RCC) {
        // TODO: make these configurable/generic?
        // or just have a massive if ... Type == Bluetooth || Led
        gpioa.moder.modify(|_, w| unsafe {
            w.moder1().bits(1)
             .moder2().bits(0b10)
             .moder3().bits(0b10)
        });
        gpioa.pupdr.modify(|_, w| unsafe {
            w.pupdr1().bits(0b01)
             .pupdr2().bits(0b01)
             .pupdr3().bits(0b01)
        });
        gpioa.afrl.modify(|_, w| unsafe { w.afrl2().bits(7).afrl3().bits(7) });
        gpioa.odr.modify(|_, w| w.odr1().clear_bit());

        rcc.apb1enr.modify(|_, w| w.usart2en().set_bit());
        rcc.ahbenr.modify(|_, w| w.dma1en().set_bit());

        self.usart.brr.modify(|_, w| unsafe { w.bits(417) });
        self.usart.cr3.modify(|_, w| w.dmat().set_bit()
                                      .dmar().set_bit());
        self.usart.cr1.modify(|_, w| {
            w.rxneie().set_bit()
             .re().set_bit()
             .te().set_bit()
             .ue().set_bit()
        });

        dma.cpar6.write(|w| unsafe { w.pa().bits(0x4000_4404) });
        dma.cmar6.write(|w| unsafe { w.ma().bits(self.receive_buffer.as_mut_ptr() as u32) });
        dma.ccr6.modify(|_, w| {
            unsafe {
                w.pl().bits(2);
            }
            w.minc().set_bit()
             .tcie().set_bit()
        });

        dma.cpar7.write(|w| unsafe { w.pa().bits(0x4000_4404) });
        dma.cmar7.write(|w| unsafe { w.ma().bits(self.send_buffer.as_mut_ptr() as u32) });
        dma.cndtr7.modify(|_, w| unsafe { w.ndt().bits(0x0) });
        dma.ccr7.modify(|_, w| {
            unsafe {
                w.pl().bits(2);
            }
            w.minc().set_bit()
             .dir().set_bit()
             .tcie().set_bit()
             .en().clear_bit()
        });
    }

    pub fn receive<F>(&mut self, dma: &mut DMA1, gpioa: &mut GPIOA, callback: F)
        where F: FnOnce(&Message)
    {
        if dma.isr.read().tcif6().bit_is_set() {
            dma.ifcr.write(|w| w.cgif6().set_bit());

            match self.receive_stage {
                ReceiveStage::Header => {
                    self.receive_stage = ReceiveStage::Body;

                    // wakeup complete, reset pa1
                    gpioa.bsrr.write(|w| w.br1().set_bit());

                    dma.ccr6.modify(|_, w| { w.en().clear_bit() });
                    dma.cmar6.write(|w| unsafe { w.ma().bits(self.receive_buffer.as_mut_ptr() as u32 + 2) });
                    dma.cndtr6.modify(|_, w| unsafe { w.ndt().bits(self.receive_buffer[1] as u16) });
                    dma.ccr6.modify(|_, w| { w.en().set_bit() });
                }
                ReceiveStage::Body => {
                    self.receive_stage = ReceiveStage::Header;
                    
                    {
                        let message = Message {
                            msg_type: self.receive_buffer[0],
                            operation: self.receive_buffer[2],
                            data: &self.receive_buffer[3..3 + self.receive_buffer[1] as usize - 1],
                        };
                        match (message.msg_type, message.operation) {
                            (6, 170) => {
                                // Wakeup acknowledged, send data
                                unsafe { dma.cndtr7.modify(|_, w| w.ndt().bits(0xb)) };
                                dma.ccr7.modify(|_, w| w.en().set_bit());
                            },
                            _ => callback(&message)
                        }
                    }

                    dma.ccr6.modify(|_, w| { w.en().clear_bit() });
                    dma.cmar6.write(|w| unsafe { w.ma().bits(self.receive_buffer.as_mut_ptr() as u32) });
                    dma.cndtr6.modify(|_, w| unsafe { w.ndt().bits(2) });
                    dma.ccr6.modify(|_, w| { w.en().set_bit() });
                }
            }
        }
    }

    pub fn send(
        &mut self,
        message_type: MsgType,
        operation: u8, // TODO: make this typed?
        data: &[u8],
        dma1: &DMA1,
        stdout: &mut Option<hio::HStdout>,
        gpioa: &GPIOA) {
        if dma1.cndtr7.read().ndt().bits() == 0 {
            self.send_buffer[0] = message_type as u8;
            self.send_buffer[1] = data.len() as u8;
            self.send_buffer[2] = operation;
            self.send_buffer[3..3 + data.len()].clone_from_slice(data);

            dma1.ccr6.modify(|_, w| { w.en().clear_bit() });
            dma1.cmar6.write(|w| unsafe { w.ma().bits(self.receive_buffer.as_mut_ptr() as u32) });
            dma1.cndtr6.modify(|_, w| unsafe { w.ndt().bits(2) });
            dma1.ccr6.modify(|_, w| { w.en().set_bit() });

            self.receive_stage = ReceiveStage::Header;

            gpioa.odr.modify(|_, w| w.odr1().clear_bit());
            gpioa.odr.modify(|_, w| w.odr1().set_bit());
        } else {
            // TODO: return an error instead
            // saying we're busy
            // using https://docs.rs/nb/0.1.1/nb/
            debug!(stdout, "tx busy").ok();
        }
    }

    pub fn tx_interrupt(&self, dma: &mut DMA1) {
        dma.ifcr.write(|w| w.cgif7().set_bit());
        dma.ccr7.modify(|_, w| w.en().clear_bit());
    }
}