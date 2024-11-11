(function() {
    var type_impls = Object.fromEntries([["teensy4_bsp",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Destination%3Cu32%3E-for-Lpspi%3CP,+N%3E\" class=\"impl\"><a href=\"#impl-Destination%3Cu32%3E-for-Lpspi%3CP,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Destination&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt; for Lpspi&lt;P, N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.destination_signal\" class=\"method trait-impl\"><a href=\"#method.destination_signal\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">destination_signal</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a></h4></section></summary><div class='docblock'>Peripheral destination request signal <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.destination_address\" class=\"method trait-impl\"><a href=\"#method.destination_address\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">destination_address</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.pointer.html\">*const </a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a></h4></section></summary><div class='docblock'>Returns a pointer to the register into which the DMA channel\nwrites data <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.enable_destination\" class=\"method trait-impl\"><a href=\"#method.enable_destination\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">enable_destination</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Perform any actions necessary to enable DMA transfers <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.disable_destination\" class=\"method trait-impl\"><a href=\"#method.disable_destination\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">disable_destination</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Perform any actions necessary to disable or cancel DMA transfers <a>Read more</a></div></details></div></details>","Destination<u32>","teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Lpspi%3CP,+N%3E\" class=\"impl\"><a href=\"#impl-Lpspi%3CP,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Lpspi&lt;P, N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.dma_write\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">dma_write</a>&lt;'a&gt;(\n    &amp;'a mut self,\n    channel: &amp;'a mut Channel,\n    buffer: &amp;'a [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>],\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Write&lt;'a, Lpspi&lt;P, N&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt;, LpspiError&gt;</h4></section></summary><div class=\"docblock\"><p>Use a DMA channel to write data to the LPSPI peripheral.</p>\n<p>The future completes when all data in <code>buffer</code> has been written to the\nperipheral. This call may block until space is available in the\ncommand queue. An error indicates that there was an issue preparing the\ntransaction, or there was an issue while waiting for space in the command\nqueue.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.dma_read\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">dma_read</a>&lt;'a&gt;(\n    &amp;'a mut self,\n    channel: &amp;'a mut Channel,\n    buffer: &amp;'a mut [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>],\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;Read&lt;'a, Lpspi&lt;P, N&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt;, LpspiError&gt;</h4></section></summary><div class=\"docblock\"><p>Use a DMA channel to read data from the LPSPI peripheral.</p>\n<p>The future completes when <code>buffer</code> is filled. This call may block until\nspace is available in the command queue. An error indicates that there was\nan issue preparing the transaction, or there was an issue waiting for space\nin the command queue.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.dma_full_duplex\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">dma_full_duplex</a>&lt;'a&gt;(\n    &amp;'a mut self,\n    rx: &amp;'a mut Channel,\n    tx: &amp;'a mut Channel,\n    buffer: &amp;'a mut [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>],\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;FullDuplex&lt;'a, Lpspi&lt;P, N&gt;, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt;, LpspiError&gt;</h4></section></summary><div class=\"docblock\"><p>Use a DMA channel to simultaneously read and write from a buffer\nand the LPSPI peripheral.</p>\n<p>The future completes when <code>buffer</code> is filled and after sending <code>buffer</code> elements.\nThis call may block until space is available in the command queue. An error\nindicates that there was an issue preparing the transaction, or there was an\nissue waiting for space in the command queue.</p>\n</div></details></div></details>",0,"teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Lpspi%3CP,+N%3E\" class=\"impl\"><a href=\"#impl-Lpspi%3CP,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Lpspi&lt;P, N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedconstant.N\" class=\"associatedconstant\"><h4 class=\"code-header\">pub const <a class=\"constant\">N</a>: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a> = N</h4></section></summary><div class=\"docblock\"><p>The peripheral instance.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_enabled\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">is_enabled</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Indicates if the driver is (<code>true</code>) or is not (<code>false</code>) enabled.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_enable\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">set_enable</a>(&amp;mut self, enable: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>)</h4></section></summary><div class=\"docblock\"><p>Enable (<code>true</code>) or disable (<code>false</code>) the peripheral.</p>\n<p>Note that disabling does not take effect immediately; instead the\nperipheral finishes the current transfer and then disables itself.\nIt is required to check <a href=\"Self::is_enabled\"><code>is_enabled()</code></a> repeatedly until the\nperipheral is actually disabled.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.reset\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">reset</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Reset the driver.</p>\n<p>Note that this may not not reset all peripheral state, like the\nenabled state.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.release\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">release</a>(self) -&gt; (Instance&lt;RegisterBlock, N&gt;, P)</h4></section></summary><div class=\"docblock\"><p>Release the SPI driver components.</p>\n<p>This does not change any component state; it releases the components as-is.\nIf you need to obtain the registers in a known, good state, consider calling\nmethods like <a href=\"Self::reset\"><code>reset()</code></a> before releasing the registers.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.bit_order\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">bit_order</a>(&amp;self) -&gt; BitOrder</h4></section></summary><div class=\"docblock\"><p>Returns the bit order configuration.</p>\n<p>See notes in <a href=\"Lpspi::set_bit_order\"><code>set_bit_order</code></a> to\nunderstand when this configuration takes effect.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_bit_order\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">set_bit_order</a>(&amp;mut self, bit_order: BitOrder)</h4></section></summary><div class=\"docblock\"><p>Set the bit order configuration.</p>\n<p>This applies to all higher-level write and transfer operations.\nIf you’re using the [<code>Transaction</code>] API with manual word reads\nand writes, set the configuration as part of the transaction.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.disabled\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">disabled</a>&lt;R&gt;(&amp;mut self, func: impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html\" title=\"trait core::ops::function::FnOnce\">FnOnce</a>(&amp;mut Disabled&lt;'_, N&gt;) -&gt; R) -&gt; R</h4></section></summary><div class=\"docblock\"><p>Temporarily disable the LPSPI peripheral.</p>\n<p>The handle to a <a href=\"crate::lpspi::Disabled\"><code>Disabled</code></a> driver lets you modify\nLPSPI settings that require a fully disabled peripheral.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.status\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">status</a>(&amp;self) -&gt; Status</h4></section></summary><div class=\"docblock\"><p>Read the status register.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clear_status\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">clear_status</a>(&amp;self, flags: Status)</h4></section></summary><div class=\"docblock\"><p>Clear the status flags.</p>\n<p>To clear status flags, set them high, then call <code>clear_status()</code>.</p>\n<p>The implementation will ensure that only the W1C bits are written, so it’s\nOK to supply <code>Status::all()</code> to clear all bits.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.interrupts\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">interrupts</a>(&amp;self) -&gt; Interrupts</h4></section></summary><div class=\"docblock\"><p>Read the interrupt enable bits.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_interrupts\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">set_interrupts</a>(&amp;self, interrupts: Interrupts)</h4></section></summary><div class=\"docblock\"><p>Set the interrupt enable bits.</p>\n<p>This writes the bits described by <code>interrupts</code> as is to the register.\nTo modify the existing interrupts flags, you should first call <a href=\"Lpspi::interrupts\"><code>interrupts</code></a>\nto get the current state, then modify that state.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clear_fifo\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">clear_fifo</a>(&amp;mut self, direction: Direction)</h4></section></summary><div class=\"docblock\"><p>Clear any existing data in the SPI receive or transfer FIFOs.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clear_fifos\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">clear_fifos</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Clear both FIFOs.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.watermark\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">watermark</a>(&amp;self, direction: Direction) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a></h4></section></summary><div class=\"docblock\"><p>Returns the watermark level for the given direction.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.fifo_status\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">fifo_status</a>(&amp;self) -&gt; FifoStatus</h4></section></summary><div class=\"docblock\"><p>Returns the FIFO status.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.read_data\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">read_data</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Read the data register.</p>\n<p>Returns <code>None</code> if the receive FIFO is empty. Otherwise, returns the complete\nread of the register. You’re reponsible for interpreting the raw value as\na data word, depending on the frame size.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.enqueue_data\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">enqueue_data</a>(&amp;self, word: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>)</h4></section></summary><div class=\"docblock\"><p>Place <code>word</code> into the transmit FIFO.</p>\n<p>This will result in the value being sent from the LPSPI.\nYou’re responsible for making sure that the transmit FIFO can\nfit this word.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_mode\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">set_mode</a>(&amp;mut self, mode: Mode)</h4></section></summary><div class=\"docblock\"><p>Set the SPI mode for the peripheral.</p>\n<p>This only affects the next transfer; ongoing transfers\nwill not be influenced.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.enqueue_transaction\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">enqueue_transaction</a>(&amp;mut self, transaction: &amp;Transaction)</h4></section></summary><div class=\"docblock\"><p>Place a transaction definition into the transmit FIFO.</p>\n<p>Once this definition is popped from the transmit FIFO, this may\naffect, or abort, any ongoing transactions.</p>\n<p>You’re responsible for making sure there’s space in the transmit\nFIFO for this transaction command.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.flush\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">flush</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, LpspiError&gt;</h4></section></summary><div class=\"docblock\"><p>Wait for all ongoing transactions to be finished.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.enable_dma_receive\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">enable_dma_receive</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Let the peripheral act as a DMA source.</p>\n<p>After this call, the peripheral will signal to the DMA engine whenever\nit has data available to read.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.disable_dma_receive\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">disable_dma_receive</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Stop the peripheral from acting as a DMA source.</p>\n<p>See the DMA chapter in the reference manual to understand when this\nshould be called in the DMA transfer lifecycle.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.enable_dma_transmit\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">enable_dma_transmit</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Let the peripheral act as a DMA destination.</p>\n<p>After this call, the peripheral will signal to the DMA engine whenever\nit has free space in its transfer buffer.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.disable_dma_transmit\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">disable_dma_transmit</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Stop the peripheral from acting as a DMA destination.</p>\n<p>See the DMA chapter in the reference manual to understand when this\nshould be called in the DMA transfer lifecycle.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.rdr\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">rdr</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.pointer.html\">*const </a>RORegister&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Produces a pointer to the receiver data register.</p>\n<p>You should use this pointer when coordinating a DMA transfer.\nYou’re not expected to read from this pointer in software.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.tdr\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">tdr</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.pointer.html\">*const </a>WORegister&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Produces a pointer to the transfer data register.</p>\n<p>You should use this pointer when coordinating a DMA transfer.\nYou’re not expected to read from this pointer in software.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.soft_reset\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">soft_reset</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Reset all internal logic while preserving the driver’s configurations.</p>\n<p>Unlike <a href=\"Self::reset\"><code>reset()</code></a>, this preserves all peripheral registers.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_watermark\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">set_watermark</a>(&amp;mut self, direction: Direction, watermark: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a></h4></section></summary><div class=\"docblock\"><p>Set the watermark level for a given direction.</p>\n<p>Returns the watermark level committed to the hardware. This may be different\nthan the supplied <code>watermark</code>, since it’s limited by the hardware.</p>\n<p>When <code>direction == Direction::Rx</code>, the receive data flag is set whenever the\nnumber of words in the receive FIFO is greater than <code>watermark</code>.</p>\n<p>When <code>direction == Direction::Tx</code>, the transmit data flag is set whenever the\nthe number of words in the transmit FIFO is less than, or equal, to <code>watermark</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clock_configs\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">clock_configs</a>(&amp;self) -&gt; ClockConfigs</h4></section></summary><div class=\"docblock\"><p>Return the driver’s clock configurations.</p>\n<p>These values are decided by calls to <a href=\"Disabled::set_clock_hz\"><code>set_clock_hz</code></a>\nand <a href=\"Disabled::set_clock_configs\"><code>set_clock_configs</code></a>.</p>\n</div></details></div></details>",0,"teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Lpspi%3CPins%3CSDO,+SDI,+SCK,+PCS0%3E,+N%3E\" class=\"impl\"><a href=\"#impl-Lpspi%3CPins%3CSDO,+SDI,+SCK,+PCS0%3E,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;SDO, SDI, SCK, PCS0, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Lpspi&lt;<a class=\"struct\" href=\"teensy4_bsp/board/struct.LpspiPins.html\" title=\"struct teensy4_bsp::board::LpspiPins\">Pins</a>&lt;SDO, SDI, SCK, PCS0&gt;, N&gt;<div class=\"where\">where\n    SDI: Pin&lt;Module = Const&lt;N&gt;, Signal = Sdi&gt;,\n    SCK: Pin&lt;Module = Const&lt;N&gt;, Signal = Sck&gt;,\n    SDO: Pin&lt;Module = Const&lt;N&gt;, Signal = Sdo&gt;,\n    PCS0: Pin&lt;Module = Const&lt;N&gt;, Signal = Pcs0&gt;,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new</a>(\n    lpspi: Instance&lt;RegisterBlock, N&gt;,\n    pins: <a class=\"struct\" href=\"teensy4_bsp/board/struct.LpspiPins.html\" title=\"struct teensy4_bsp::board::LpspiPins\">Pins</a>&lt;SDO, SDI, SCK, PCS0&gt;,\n) -&gt; Lpspi&lt;<a class=\"struct\" href=\"teensy4_bsp/board/struct.LpspiPins.html\" title=\"struct teensy4_bsp::board::LpspiPins\">Pins</a>&lt;SDO, SDI, SCK, PCS0&gt;, N&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new LPSPI driver from the RAL LPSPI instance and a set of pins.</p>\n<p>When this call returns, the LPSPI pins are configured for their function.\nThe peripheral is enabled after reset. The LPSPI clock speed is unspecified.\nThe mode is [<code>MODE_0</code>]. The sample point is [<code>SamplePoint::DelayedEdge</code>].</p>\n</div></details></div></details>",0,"teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Source%3Cu32%3E-for-Lpspi%3CP,+N%3E\" class=\"impl\"><a href=\"#impl-Source%3Cu32%3E-for-Lpspi%3CP,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Source&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt; for Lpspi&lt;P, N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.source_signal\" class=\"method trait-impl\"><a href=\"#method.source_signal\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">source_signal</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a></h4></section></summary><div class='docblock'>Peripheral source request signal <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.source_address\" class=\"method trait-impl\"><a href=\"#method.source_address\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">source_address</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.pointer.html\">*const </a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a></h4></section></summary><div class='docblock'>Returns a pointer to the register from which the DMA channel\nreads data <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.enable_source\" class=\"method trait-impl\"><a href=\"#method.enable_source\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">enable_source</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Perform any actions necessary to enable DMA transfers <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.disable_source\" class=\"method trait-impl\"><a href=\"#method.disable_source\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">disable_source</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Perform any actions necessary to disable or cancel DMA transfers <a>Read more</a></div></details></div></details>","Source<u32>","teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Transfer%3Cu16%3E-for-Lpspi%3CP,+N%3E\" class=\"impl\"><a href=\"#impl-Transfer%3Cu16%3E-for-Lpspi%3CP,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Transfer&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u16.html\">u16</a>&gt; for Lpspi&lt;P, N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Error</a> = LpspiError</h4></section></summary><div class='docblock'>Error type</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.transfer\" class=\"method trait-impl\"><a href=\"#method.transfer\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">transfer</a>&lt;'a&gt;(\n    &amp;mut self,\n    words: &amp;'a mut [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u16.html\">u16</a>],\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;&amp;'a [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u16.html\">u16</a>], &lt;Lpspi&lt;P, N&gt; as Transfer&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u16.html\">u16</a>&gt;&gt;::Error&gt;</h4></section></summary><div class='docblock'>Sends <code>words</code> to the slave. Returns the <code>words</code> received from the slave</div></details></div></details>","Transfer<u16>","teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Transfer%3Cu32%3E-for-Lpspi%3CP,+N%3E\" class=\"impl\"><a href=\"#impl-Transfer%3Cu32%3E-for-Lpspi%3CP,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Transfer&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt; for Lpspi&lt;P, N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Error</a> = LpspiError</h4></section></summary><div class='docblock'>Error type</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.transfer\" class=\"method trait-impl\"><a href=\"#method.transfer\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">transfer</a>&lt;'a&gt;(\n    &amp;mut self,\n    words: &amp;'a mut [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>],\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;&amp;'a [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>], &lt;Lpspi&lt;P, N&gt; as Transfer&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt;&gt;::Error&gt;</h4></section></summary><div class='docblock'>Sends <code>words</code> to the slave. Returns the <code>words</code> received from the slave</div></details></div></details>","Transfer<u32>","teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Transfer%3Cu8%3E-for-Lpspi%3CP,+N%3E\" class=\"impl\"><a href=\"#impl-Transfer%3Cu8%3E-for-Lpspi%3CP,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Transfer&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; for Lpspi&lt;P, N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Error</a> = LpspiError</h4></section></summary><div class='docblock'>Error type</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.transfer\" class=\"method trait-impl\"><a href=\"#method.transfer\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">transfer</a>&lt;'a&gt;(\n    &amp;mut self,\n    words: &amp;'a mut [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>],\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;&amp;'a [<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>], &lt;Lpspi&lt;P, N&gt; as Transfer&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt;&gt;::Error&gt;</h4></section></summary><div class='docblock'>Sends <code>words</code> to the slave. Returns the <code>words</code> received from the slave</div></details></div></details>","Transfer<u8>","teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Write%3Cu16%3E-for-Lpspi%3CP,+N%3E\" class=\"impl\"><a href=\"#impl-Write%3Cu16%3E-for-Lpspi%3CP,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Write&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u16.html\">u16</a>&gt; for Lpspi&lt;P, N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Error</a> = LpspiError</h4></section></summary><div class='docblock'>Error type</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write\" class=\"method trait-impl\"><a href=\"#method.write\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">write</a>(\n    &amp;mut self,\n    words: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u16.html\">u16</a>],\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, &lt;Lpspi&lt;P, N&gt; as Write&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u16.html\">u16</a>&gt;&gt;::Error&gt;</h4></section></summary><div class='docblock'>Sends <code>words</code> to the slave, ignoring all the incoming words</div></details></div></details>","Write<u16>","teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Write%3Cu32%3E-for-Lpspi%3CP,+N%3E\" class=\"impl\"><a href=\"#impl-Write%3Cu32%3E-for-Lpspi%3CP,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Write&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt; for Lpspi&lt;P, N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Error</a> = LpspiError</h4></section></summary><div class='docblock'>Error type</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write\" class=\"method trait-impl\"><a href=\"#method.write\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">write</a>(\n    &amp;mut self,\n    words: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>],\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, &lt;Lpspi&lt;P, N&gt; as Write&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt;&gt;::Error&gt;</h4></section></summary><div class='docblock'>Sends <code>words</code> to the slave, ignoring all the incoming words</div></details></div></details>","Write<u32>","teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Write%3Cu8%3E-for-Lpspi%3CP,+N%3E\" class=\"impl\"><a href=\"#impl-Write%3Cu8%3E-for-Lpspi%3CP,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Write&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; for Lpspi&lt;P, N&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Error</a> = LpspiError</h4></section></summary><div class='docblock'>Error type</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write\" class=\"method trait-impl\"><a href=\"#method.write\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">write</a>(\n    &amp;mut self,\n    words: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>],\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, &lt;Lpspi&lt;P, N&gt; as Write&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt;&gt;::Error&gt;</h4></section></summary><div class='docblock'>Sends <code>words</code> to the slave, ignoring all the incoming words</div></details></div></details>","Write<u8>","teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"],["<section id=\"impl-Bidirectional%3Cu32%3E-for-Lpspi%3CP,+N%3E\" class=\"impl\"><a href=\"#impl-Bidirectional%3Cu32%3E-for-Lpspi%3CP,+N%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u8.html\">u8</a>&gt; Bidirectional&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>&gt; for Lpspi&lt;P, N&gt;</h3></section>","Bidirectional<u32>","teensy4_bsp::board::Lpspi1","teensy4_bsp::board::Lpspi2","teensy4_bsp::board::Lpspi3","teensy4_bsp::board::Lpspi4"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[38779]}