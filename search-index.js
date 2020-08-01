var searchIndex = JSON.parse('{\
"teensy4_bsp":{"doc":"A Rust board support package (BSP) for the Teensy 4.","i":[[4,"interrupt","teensy4_bsp","Available interrupts for this device",null,null],[13,"DMA0_DMA16","","0:",0,null],[13,"DMA1_DMA17","","1:",0,null],[13,"DMA2_DMA18","","2:",0,null],[13,"DMA3_DMA19","","3:",0,null],[13,"DMA4_DMA20","","4:",0,null],[13,"DMA5_DMA21","","5:",0,null],[13,"DMA6_DMA22","","6:",0,null],[13,"DMA7_DMA23","","7:",0,null],[13,"DMA8_DMA24","","8:",0,null],[13,"DMA9_DMA25","","9:",0,null],[13,"DMA10_DMA26","","10:",0,null],[13,"DMA11_DMA27","","11:",0,null],[13,"DMA12_DMA28","","12:",0,null],[13,"DMA13_DMA29","","13:",0,null],[13,"DMA14_DMA30","","14:",0,null],[13,"DMA15_DMA31","","15:",0,null],[13,"DMA_ERROR","","16:",0,null],[13,"CTI0_ERROR","","17:",0,null],[13,"CTI1_ERROR","","18:",0,null],[13,"CORE","","19:",0,null],[13,"LPUART1","","20:",0,null],[13,"LPUART2","","21:",0,null],[13,"LPUART3","","22:",0,null],[13,"LPUART4","","23:",0,null],[13,"LPUART5","","24:",0,null],[13,"LPUART6","","25:",0,null],[13,"LPUART7","","26:",0,null],[13,"LPUART8","","27:",0,null],[13,"LPI2C1","","28:",0,null],[13,"LPI2C2","","29:",0,null],[13,"LPI2C3","","30:",0,null],[13,"LPI2C4","","31:",0,null],[13,"LPSPI1","","32:",0,null],[13,"LPSPI2","","33:",0,null],[13,"LPSPI3","","34:",0,null],[13,"LPSPI4","","35:",0,null],[13,"CAN1","","36:",0,null],[13,"CAN2","","37:",0,null],[13,"FLEXRAM","","38:",0,null],[13,"KPP","","39:",0,null],[13,"TSC_DIG","","40:",0,null],[13,"GPR_IRQ","","41:",0,null],[13,"LCDIF","","42:",0,null],[13,"CSI","","43:",0,null],[13,"PXP","","44:",0,null],[13,"WDOG2","","45:",0,null],[13,"SNVS_HP_WRAPPER","","46:",0,null],[13,"SNVS_HP_WRAPPER_TZ","","47:",0,null],[13,"SNVS_LP_WRAPPER","","48:",0,null],[13,"CSU","","49:",0,null],[13,"DCP","","50:",0,null],[13,"DCP_VMI","","51:",0,null],[13,"Reserved68","","52:",0,null],[13,"TRNG","","53:",0,null],[13,"SJC","","54:",0,null],[13,"BEE","","55:",0,null],[13,"SAI1","","56:",0,null],[13,"SAI2","","57:",0,null],[13,"SAI3_RX","","58:",0,null],[13,"SAI3_TX","","59:",0,null],[13,"SPDIF","","60:",0,null],[13,"PMU_EVENT","","61:",0,null],[13,"Reserved78","","62:",0,null],[13,"TEMP_LOW_HIGH","","63:",0,null],[13,"TEMP_PANIC","","64:",0,null],[13,"USB_PHY1","","65:",0,null],[13,"USB_PHY2","","66:",0,null],[13,"ADC1","","67:",0,null],[13,"ADC2","","68:",0,null],[13,"DCDC","","69:",0,null],[13,"Reserved86","","70:",0,null],[13,"Reserved87","","71:",0,null],[13,"GPIO1_INT0","","72:",0,null],[13,"GPIO1_INT1","","73:",0,null],[13,"GPIO1_INT2","","74:",0,null],[13,"GPIO1_INT3","","75:",0,null],[13,"GPIO1_INT4","","76:",0,null],[13,"GPIO1_INT5","","77:",0,null],[13,"GPIO1_INT6","","78:",0,null],[13,"GPIO1_INT7","","79:",0,null],[13,"GPIO1_Combined_0_15","","80:",0,null],[13,"GPIO1_Combined_16_31","","81:",0,null],[13,"GPIO2_Combined_0_15","","82:",0,null],[13,"GPIO2_Combined_16_31","","83:",0,null],[13,"GPIO3_Combined_0_15","","84:",0,null],[13,"GPIO3_Combined_16_31","","85:",0,null],[13,"GPIO4_Combined_0_15","","86:",0,null],[13,"GPIO4_Combined_16_31","","87:",0,null],[13,"GPIO5_Combined_0_15","","88:",0,null],[13,"GPIO5_Combined_16_31","","89:",0,null],[13,"FLEXIO1","","90:",0,null],[13,"FLEXIO2","","91:",0,null],[13,"WDOG1","","92:",0,null],[13,"RTWDOG","","93:",0,null],[13,"EWM","","94:",0,null],[13,"CCM_1","","95:",0,null],[13,"CCM_2","","96:",0,null],[13,"GPC","","97:",0,null],[13,"SRC","","98:",0,null],[13,"Reserved115","","99:",0,null],[13,"GPT1","","100:",0,null],[13,"GPT2","","101:",0,null],[13,"PWM1_0","","102:",0,null],[13,"PWM1_1","","103:",0,null],[13,"PWM1_2","","104:",0,null],[13,"PWM1_3","","105:",0,null],[13,"PWM1_FAULT","","106:",0,null],[13,"FLEXSPI2","","107:",0,null],[13,"FLEXSPI","","108:",0,null],[13,"SEMC","","109:",0,null],[13,"USDHC1","","110:",0,null],[13,"USDHC2","","111:",0,null],[13,"USB_OTG2","","112:",0,null],[13,"USB_OTG1","","113:",0,null],[13,"ENET","","114:",0,null],[13,"ENET_1588_Timer","","115:",0,null],[13,"XBAR1_IRQ_0_1","","116:",0,null],[13,"XBAR1_IRQ_2_3","","117:",0,null],[13,"ADC_ETC_IRQ0","","118:",0,null],[13,"ADC_ETC_IRQ1","","119:",0,null],[13,"ADC_ETC_IRQ2","","120:",0,null],[13,"ADC_ETC_ERROR_IRQ","","121:",0,null],[13,"PIT","","122:",0,null],[13,"ACMP1","","123:",0,null],[13,"ACMP2","","124:",0,null],[13,"ACMP3","","125:",0,null],[13,"ACMP4","","126:",0,null],[13,"Reserved143","","127:",0,null],[13,"Reserved144","","128:",0,null],[13,"ENC1","","129:",0,null],[13,"ENC2","","130:",0,null],[13,"ENC3","","131:",0,null],[13,"ENC4","","132:",0,null],[13,"TMR1","","133:",0,null],[13,"TMR2","","134:",0,null],[13,"TMR3","","135:",0,null],[13,"TMR4","","136:",0,null],[13,"PWM2_0","","137:",0,null],[13,"PWM2_1","","138:",0,null],[13,"PWM2_2","","139:",0,null],[13,"PWM2_3","","140:",0,null],[13,"PWM2_FAULT","","141:",0,null],[13,"PWM3_0","","142:",0,null],[13,"PWM3_1","","143:",0,null],[13,"PWM3_2","","144:",0,null],[13,"PWM3_3","","145:",0,null],[13,"PWM3_FAULT","","146:",0,null],[13,"PWM4_0","","147:",0,null],[13,"PWM4_1","","148:",0,null],[13,"PWM4_2","","149:",0,null],[13,"PWM4_3","","150:",0,null],[13,"PWM4_FAULT","","151:",0,null],[13,"ENET2","","152:",0,null],[13,"ENET2_1588_Timer","","153:",0,null],[13,"CAN3","","154:",0,null],[13,"Reserved171","","155:",0,null],[13,"FLEXIO3","","156:",0,null],[13,"GPIO6_7_8_9","","157:",0,null],[4,"Interrupt","","Available interrupts for this device",null,null],[13,"DMA0_DMA16","","0:",0,null],[13,"DMA1_DMA17","","1:",0,null],[13,"DMA2_DMA18","","2:",0,null],[13,"DMA3_DMA19","","3:",0,null],[13,"DMA4_DMA20","","4:",0,null],[13,"DMA5_DMA21","","5:",0,null],[13,"DMA6_DMA22","","6:",0,null],[13,"DMA7_DMA23","","7:",0,null],[13,"DMA8_DMA24","","8:",0,null],[13,"DMA9_DMA25","","9:",0,null],[13,"DMA10_DMA26","","10:",0,null],[13,"DMA11_DMA27","","11:",0,null],[13,"DMA12_DMA28","","12:",0,null],[13,"DMA13_DMA29","","13:",0,null],[13,"DMA14_DMA30","","14:",0,null],[13,"DMA15_DMA31","","15:",0,null],[13,"DMA_ERROR","","16:",0,null],[13,"CTI0_ERROR","","17:",0,null],[13,"CTI1_ERROR","","18:",0,null],[13,"CORE","","19:",0,null],[13,"LPUART1","","20:",0,null],[13,"LPUART2","","21:",0,null],[13,"LPUART3","","22:",0,null],[13,"LPUART4","","23:",0,null],[13,"LPUART5","","24:",0,null],[13,"LPUART6","","25:",0,null],[13,"LPUART7","","26:",0,null],[13,"LPUART8","","27:",0,null],[13,"LPI2C1","","28:",0,null],[13,"LPI2C2","","29:",0,null],[13,"LPI2C3","","30:",0,null],[13,"LPI2C4","","31:",0,null],[13,"LPSPI1","","32:",0,null],[13,"LPSPI2","","33:",0,null],[13,"LPSPI3","","34:",0,null],[13,"LPSPI4","","35:",0,null],[13,"CAN1","","36:",0,null],[13,"CAN2","","37:",0,null],[13,"FLEXRAM","","38:",0,null],[13,"KPP","","39:",0,null],[13,"TSC_DIG","","40:",0,null],[13,"GPR_IRQ","","41:",0,null],[13,"LCDIF","","42:",0,null],[13,"CSI","","43:",0,null],[13,"PXP","","44:",0,null],[13,"WDOG2","","45:",0,null],[13,"SNVS_HP_WRAPPER","","46:",0,null],[13,"SNVS_HP_WRAPPER_TZ","","47:",0,null],[13,"SNVS_LP_WRAPPER","","48:",0,null],[13,"CSU","","49:",0,null],[13,"DCP","","50:",0,null],[13,"DCP_VMI","","51:",0,null],[13,"Reserved68","","52:",0,null],[13,"TRNG","","53:",0,null],[13,"SJC","","54:",0,null],[13,"BEE","","55:",0,null],[13,"SAI1","","56:",0,null],[13,"SAI2","","57:",0,null],[13,"SAI3_RX","","58:",0,null],[13,"SAI3_TX","","59:",0,null],[13,"SPDIF","","60:",0,null],[13,"PMU_EVENT","","61:",0,null],[13,"Reserved78","","62:",0,null],[13,"TEMP_LOW_HIGH","","63:",0,null],[13,"TEMP_PANIC","","64:",0,null],[13,"USB_PHY1","","65:",0,null],[13,"USB_PHY2","","66:",0,null],[13,"ADC1","","67:",0,null],[13,"ADC2","","68:",0,null],[13,"DCDC","","69:",0,null],[13,"Reserved86","","70:",0,null],[13,"Reserved87","","71:",0,null],[13,"GPIO1_INT0","","72:",0,null],[13,"GPIO1_INT1","","73:",0,null],[13,"GPIO1_INT2","","74:",0,null],[13,"GPIO1_INT3","","75:",0,null],[13,"GPIO1_INT4","","76:",0,null],[13,"GPIO1_INT5","","77:",0,null],[13,"GPIO1_INT6","","78:",0,null],[13,"GPIO1_INT7","","79:",0,null],[13,"GPIO1_Combined_0_15","","80:",0,null],[13,"GPIO1_Combined_16_31","","81:",0,null],[13,"GPIO2_Combined_0_15","","82:",0,null],[13,"GPIO2_Combined_16_31","","83:",0,null],[13,"GPIO3_Combined_0_15","","84:",0,null],[13,"GPIO3_Combined_16_31","","85:",0,null],[13,"GPIO4_Combined_0_15","","86:",0,null],[13,"GPIO4_Combined_16_31","","87:",0,null],[13,"GPIO5_Combined_0_15","","88:",0,null],[13,"GPIO5_Combined_16_31","","89:",0,null],[13,"FLEXIO1","","90:",0,null],[13,"FLEXIO2","","91:",0,null],[13,"WDOG1","","92:",0,null],[13,"RTWDOG","","93:",0,null],[13,"EWM","","94:",0,null],[13,"CCM_1","","95:",0,null],[13,"CCM_2","","96:",0,null],[13,"GPC","","97:",0,null],[13,"SRC","","98:",0,null],[13,"Reserved115","","99:",0,null],[13,"GPT1","","100:",0,null],[13,"GPT2","","101:",0,null],[13,"PWM1_0","","102:",0,null],[13,"PWM1_1","","103:",0,null],[13,"PWM1_2","","104:",0,null],[13,"PWM1_3","","105:",0,null],[13,"PWM1_FAULT","","106:",0,null],[13,"FLEXSPI2","","107:",0,null],[13,"FLEXSPI","","108:",0,null],[13,"SEMC","","109:",0,null],[13,"USDHC1","","110:",0,null],[13,"USDHC2","","111:",0,null],[13,"USB_OTG2","","112:",0,null],[13,"USB_OTG1","","113:",0,null],[13,"ENET","","114:",0,null],[13,"ENET_1588_Timer","","115:",0,null],[13,"XBAR1_IRQ_0_1","","116:",0,null],[13,"XBAR1_IRQ_2_3","","117:",0,null],[13,"ADC_ETC_IRQ0","","118:",0,null],[13,"ADC_ETC_IRQ1","","119:",0,null],[13,"ADC_ETC_IRQ2","","120:",0,null],[13,"ADC_ETC_ERROR_IRQ","","121:",0,null],[13,"PIT","","122:",0,null],[13,"ACMP1","","123:",0,null],[13,"ACMP2","","124:",0,null],[13,"ACMP3","","125:",0,null],[13,"ACMP4","","126:",0,null],[13,"Reserved143","","127:",0,null],[13,"Reserved144","","128:",0,null],[13,"ENC1","","129:",0,null],[13,"ENC2","","130:",0,null],[13,"ENC3","","131:",0,null],[13,"ENC4","","132:",0,null],[13,"TMR1","","133:",0,null],[13,"TMR2","","134:",0,null],[13,"TMR3","","135:",0,null],[13,"TMR4","","136:",0,null],[13,"PWM2_0","","137:",0,null],[13,"PWM2_1","","138:",0,null],[13,"PWM2_2","","139:",0,null],[13,"PWM2_3","","140:",0,null],[13,"PWM2_FAULT","","141:",0,null],[13,"PWM3_0","","142:",0,null],[13,"PWM3_1","","143:",0,null],[13,"PWM3_2","","144:",0,null],[13,"PWM3_3","","145:",0,null],[13,"PWM3_FAULT","","146:",0,null],[13,"PWM4_0","","147:",0,null],[13,"PWM4_1","","148:",0,null],[13,"PWM4_2","","149:",0,null],[13,"PWM4_3","","150:",0,null],[13,"PWM4_FAULT","","151:",0,null],[13,"ENET2","","152:",0,null],[13,"ENET2_1588_Timer","","153:",0,null],[13,"CAN3","","154:",0,null],[13,"Reserved171","","155:",0,null],[13,"FLEXIO3","","156:",0,null],[13,"GPIO6_7_8_9","","157:",0,null],[17,"NVIC_PRIO_BITS","","Number of priority bits implemented by the NVIC",null,null],[3,"SysTick","","A type that represents the system timer, SYSTICK",null,null],[3,"Pins","","Teensy pins that do not yet have a function",null,null],[12,"p0","","Pin 0",1,null],[12,"p1","","Pin 1",1,null],[12,"p2","","Pin 2",1,null],[12,"p3","","Pin 3",1,null],[12,"p4","","Pin 4",1,null],[12,"p5","","Pin 5",1,null],[12,"p6","","Pin 6",1,null],[12,"p7","","Pin 7",1,null],[12,"p8","","Pin 8",1,null],[12,"p9","","Pin 9",1,null],[12,"p10","","Pin 10",1,null],[12,"p11","","Pin 11",1,null],[12,"p12","","Pin 12",1,null],[12,"p13","","Pin 13",1,null],[12,"p14","","Pin 14",1,null],[12,"p15","","Pin 15",1,null],[12,"p16","","Pin 16",1,null],[12,"p17","","Pin 17",1,null],[12,"p18","","Pin 18",1,null],[12,"p19","","Pin 19",1,null],[12,"p20","","Pin 20",1,null],[12,"p21","","Pin 21",1,null],[12,"p22","","Pin 22",1,null],[12,"p23","","Pin 23",1,null],[12,"p24","","Pin 24",1,null],[12,"p25","","Pin 25",1,null],[12,"p28","","Pin 28",1,null],[12,"p29","","Pin 29",1,null],[12,"p33","","Pin 33",1,null],[12,"p36","","Pin 36",1,null],[12,"p37","","Pin 37",1,null],[3,"Peripherals","","All peripherals available on the Teensy4",null,null],[12,"ccm","","Clock control module (forwarded from the HAL)",2,null],[12,"pit","","PIT timers (forwarded from the HAL)",2,null],[12,"usb","","The USB logger and serial reader",2,null],[12,"dcdc","","DCDC converters",2,null],[12,"pwm1","","PWM1 controller",2,null],[12,"pwm2","","PWM2 controller",2,null],[12,"pwm3","","PWM3 controller",2,null],[12,"pwm4","","PWM4 controller",2,null],[12,"pins","","Teensy pins",2,null],[12,"i2c","","Unclocked I2C peripherals",2,null],[12,"spi","","Unclocked SPI peripherals",2,null],[12,"uart","","Unclocked UART peripherals",2,null],[12,"gpr","","General purpose registers, used when configuring GPIO pins.",2,null],[12,"gpt1","","General purpose timer 1",2,null],[12,"gpt2","","General purpose timer 2",2,null],[12,"dma","","DMA channels",2,null],[12,"systick","","The SysTick delay timer",2,null],[5,"configure_led","","Configure the board\'s LED",null,[[["gpio_b0_03",3],["gpr",3]],["led",6]]],[11,"delay","","Blocks for `ms` milliseconds",3,[[]]],[0,"usb","","Teensy 4 USB, taken from the original Teensy 4 C libraries.",null,null],[3,"LoggingConfig","teensy4_bsp::usb","Logging configuration",null,null],[12,"max_level","","The max log level",4,null],[12,"filters","","A list of filtered targets to log.",4,null],[3,"USB","","A handle that enables USB I/O",null,null],[3,"Writer","","A type that can send data to a USB serial host",null,null],[3,"Reader","","A type that can read USB serial messages from a host",null,null],[11,"init","","Initializes the USB stack. This prepares the logging…",5,[[["loggingconfig",3]],["reader",3]]],[11,"split","","Split the USB handle into reader and writer halves",5,[[]]],[11,"write","","Writes raw bytes to the USB serial host",6,[[["asref",8]]]],[11,"read","","Read from the USB serial endpoint into buffer. Returns the…",7,[[]]],[6,"LED","teensy4_bsp","The LED in its final configuration",null,null],[11,"take","","Instantiate the system peripherals. This may only be…",2,[[],["option",4]]],[11,"steal","","Steal all of the HAL\'s peripherals.",2,[[]]],[11,"from","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"into","","",0,[[]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"into","","",3,[[]]],[11,"try_into","","",3,[[],["result",4]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"into","","",1,[[]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"into","","",2,[[]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","teensy4_bsp::usb","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"into","","",4,[[]]],[11,"try_into","","",4,[[],["result",4]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"into","","",5,[[]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"into","","",6,[[]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"into","","",7,[[]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"clone","teensy4_bsp","",0,[[],["interrupt",4]]],[11,"nr","","",0,[[]]],[11,"write_str","teensy4_bsp::usb","",6,[[],["result",6]]],[11,"default","","",4,[[],["loggingconfig",3]]],[11,"delay_ms","teensy4_bsp","",3,[[]]],[11,"delay_ms","","",3,[[]]],[11,"delay_ms","","",3,[[]]]],"p":[[4,"Interrupt"],[3,"Pins"],[3,"Peripherals"],[3,"SysTick"],[3,"LoggingConfig"],[3,"USB"],[3,"Writer"],[3,"Reader"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);