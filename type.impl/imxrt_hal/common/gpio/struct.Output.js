(function() {
    var type_impls = Object.fromEntries([["teensy4_bsp",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ErrorType-for-Output%3CP%3E\" class=\"impl\"><a href=\"#impl-ErrorType-for-Output%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; ErrorType for Output&lt;P&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Error</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html\" title=\"enum core::convert::Infallible\">Infallible</a></h4></section></summary><div class='docblock'>Error type</div></details></div></details>","ErrorType","teensy4_bsp::board::Led"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-InputPin-for-Output%3CP%3E\" class=\"impl\"><a href=\"#impl-InputPin-for-Output%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; InputPin for Output&lt;P&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_high\" class=\"method trait-impl\"><a href=\"#method.is_high\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_high</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>, &lt;Output&lt;P&gt; as ErrorType&gt;::Error&gt;</h4></section></summary><div class='docblock'>Is the input pin high?</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_low\" class=\"method trait-impl\"><a href=\"#method.is_low\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_low</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>, &lt;Output&lt;P&gt; as ErrorType&gt;::Error&gt;</h4></section></summary><div class='docblock'>Is the input pin low?</div></details></div></details>","InputPin","teensy4_bsp::board::Led"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Output%3CP%3E\" class=\"impl\"><a href=\"#impl-Output%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; Output&lt;P&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.set\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">set</a>(&amp;self)</h4></section></summary><div class=\"docblock\"><p>Set the GPIO high.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clear\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">clear</a>(&amp;self)</h4></section></summary><div class=\"docblock\"><p>Set the GPIO low.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.toggle\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">toggle</a>(&amp;self)</h4></section></summary><div class=\"docblock\"><p>Alternate the GPIO pin output.</p>\n<p><code>toggle</code> is implemented in hardware, so it will be more efficient\nthan implementing in software.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_set\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">is_set</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Returns <code>true</code> if the GPIO is set.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_pad_high\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">is_pad_high</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class=\"docblock\"><p>Returns <code>true</code> if the value of the pad is high.</p>\n<p>Can differ from <a href=\"Self::is_set\"><code>is_set()</code></a>, especially in an open drain config.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.release\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">release</a>(self) -&gt; P</h4></section></summary><div class=\"docblock\"><p>Release the underlying pin object.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.pin\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">pin</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;P</a></h4></section></summary><div class=\"docblock\"><p>Access the underlying pin.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.pin_mut\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">pin_mut</a>(&amp;mut self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;mut P</a></h4></section></summary><div class=\"docblock\"><p>Mutably access the underling pin.</p>\n</div></details></div></details>",0,"teensy4_bsp::board::Led"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OutputPin-for-Output%3CP%3E\" class=\"impl\"><a href=\"#impl-OutputPin-for-Output%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; OutputPin for Output&lt;P&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_high\" class=\"method trait-impl\"><a href=\"#method.set_high\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">set_high</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, &lt;Output&lt;P&gt; as ErrorType&gt;::Error&gt;</h4></section></summary><div class='docblock'>Drives the pin high. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_low\" class=\"method trait-impl\"><a href=\"#method.set_low\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">set_low</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, &lt;Output&lt;P&gt; as ErrorType&gt;::Error&gt;</h4></section></summary><div class='docblock'>Drives the pin low. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_state\" class=\"method trait-impl\"><a href=\"#method.set_state\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">set_state</a>(&amp;mut self, state: PinState) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, Self::Error&gt;</h4></section></summary><div class='docblock'>Drives the pin high or low depending on the provided value. <a>Read more</a></div></details></div></details>","OutputPin","teensy4_bsp::board::Led"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-OutputPin-for-Output%3CP%3E\" class=\"impl\"><a href=\"#impl-OutputPin-for-Output%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; OutputPin for Output&lt;P&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Error\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Error\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Error</a> = <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html\" title=\"enum core::convert::Infallible\">Infallible</a></h4></section></summary><div class='docblock'>Error type</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_high\" class=\"method trait-impl\"><a href=\"#method.set_high\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">set_high</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, &lt;Output&lt;P&gt; as OutputPin&gt;::Error&gt;</h4></section></summary><div class='docblock'>Drives the pin high <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_low\" class=\"method trait-impl\"><a href=\"#method.set_low\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">set_low</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, &lt;Output&lt;P&gt; as OutputPin&gt;::Error&gt;</h4></section></summary><div class='docblock'>Drives the pin low <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_state\" class=\"method trait-impl\"><a href=\"#method.set_state\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">set_state</a>(&amp;mut self, state: PinState) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, Self::Error&gt;</h4></section></summary><div class='docblock'>Drives the pin high or low depending on the provided value <a>Read more</a></div></details></div></details>","OutputPin","teensy4_bsp::board::Led"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-StatefulOutputPin-for-Output%3CP%3E\" class=\"impl\"><a href=\"#impl-StatefulOutputPin-for-Output%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; StatefulOutputPin for Output&lt;P&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_set_high\" class=\"method trait-impl\"><a href=\"#method.is_set_high\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_set_high</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>, &lt;Output&lt;P&gt; as ErrorType&gt;::Error&gt;</h4></section></summary><div class='docblock'>Is the pin in drive high mode? <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_set_low\" class=\"method trait-impl\"><a href=\"#method.is_set_low\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_set_low</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>, &lt;Output&lt;P&gt; as ErrorType&gt;::Error&gt;</h4></section></summary><div class='docblock'>Is the pin in drive low mode? <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.toggle\" class=\"method trait-impl\"><a href=\"#method.toggle\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">toggle</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, &lt;Output&lt;P&gt; as ErrorType&gt;::Error&gt;</h4></section></summary><div class='docblock'>Toggle pin output.</div></details></div></details>","StatefulOutputPin","teensy4_bsp::board::Led"],["<section id=\"impl-Send-for-Output%3CP%3E\" class=\"impl\"><a href=\"#impl-Send-for-Output%3CP%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for Output&lt;P&gt;<div class=\"where\">where\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,</div></h3></section>","Send","teensy4_bsp::board::Led"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[13557]}