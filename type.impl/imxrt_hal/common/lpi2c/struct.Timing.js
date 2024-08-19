(function() {var type_impls = {
"teensy4_bsp":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-Timing\" class=\"impl\"><a href=\"#impl-Clone-for-Timing\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for Timing</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; Timing</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/clone.rs.html#174\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","teensy4_bsp::clock_power::Lpi2cBaud"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Timing\" class=\"impl\"><a href=\"#impl-Debug-for-Timing\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for Timing</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","teensy4_bsp::clock_power::Lpi2cBaud"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-Timing\" class=\"impl\"><a href=\"#impl-PartialEq-for-Timing\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for Timing</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, other: &amp;Timing) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>self</code> and <code>other</code> values to be equal, and is used by <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#261\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Tests for <code>!=</code>. The default implementation is almost always sufficient,\nand should not be overridden without very good reason.</div></details></div></details>","PartialEq","teensy4_bsp::clock_power::Lpi2cBaud"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Timing\" class=\"impl\"><a href=\"#impl-Timing\" class=\"anchor\">§</a><h3 class=\"code-header\">impl Timing</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.ideal\" class=\"method\"><h4 class=\"code-header\">pub const fn <a class=\"fn\">ideal</a>(clock_hz: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>, clock_speed: <a class=\"enum\" href=\"teensy4_bsp/board/enum.Lpi2cClockSpeed.html\" title=\"enum teensy4_bsp::board::Lpi2cClockSpeed\">ClockSpeed</a>) -&gt; Timing</h4></section></summary><div class=\"docblock\"><p>Compute timing parameters assuming an ideal I2C bus.</p>\n<p>This constructor assumes that</p>\n<ul>\n<li>the SDA / SCL rise times are negligible (take less than one functional clock cycle).</li>\n<li>there’s no need for glitch filters (FLITSCL = FILTSDA = 0).</li>\n</ul>\n<p>These assumptions may not hold true for high clock speeds and I2C bus loadings.\nIf that’s the case, you may find it’s better to define timing parameters yourself.</p>\n<p>Note that this function can run at compile time. Consider evaluating in a const\ncontext to avoid the possibility of panics.</p>\n<h5 id=\"panics\"><a class=\"doc-anchor\" href=\"#panics\">§</a>Panics</h5>\n<p>After evaluating all prescalars, this function panics if the computed clock period\ncannot be represented in the 6 bits available for the configuration.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><h4 class=\"code-header\">pub const fn <a class=\"fn\">new</a>(\n    clock_configuration: ClockConfiguration,\n    prescaler: Prescaler,\n) -&gt; Timing</h4></section></summary><div class=\"docblock\"><p>Computes timing parameters assuming an ideal circuit.</p>\n<p>Define LPI2C timings by the clock configuration values, and a prescaler.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clock_configuration\" class=\"method\"><h4 class=\"code-header\">pub const fn <a class=\"fn\">clock_configuration</a>(&amp;self) -&gt; ClockConfiguration</h4></section></summary><div class=\"docblock\"><p>Returns the clock configuration.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.prescaler\" class=\"method\"><h4 class=\"code-header\">pub const fn <a class=\"fn\">prescaler</a>(&amp;self) -&gt; Prescaler</h4></section></summary><div class=\"docblock\"><p>Returns the prescaler.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.override_busidle\" class=\"method\"><h4 class=\"code-header\">pub const fn <a class=\"fn\">override_busidle</a>(self, busidle: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.u32.html\">u32</a>) -&gt; Timing</h4></section></summary><div class=\"docblock\"><p>Override the BUSIDLE parameter.</p>\n<p>The minimum BUSIDLE is computed by CLKLO, SETHOLD, and CLKHI. Use\nthis method to override the value.</p>\n</div></details></div></details>",0,"teensy4_bsp::clock_power::Lpi2cBaud"],["<section id=\"impl-Copy-for-Timing\" class=\"impl\"><a href=\"#impl-Copy-for-Timing\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for Timing</h3></section>","Copy","teensy4_bsp::clock_power::Lpi2cBaud"],["<section id=\"impl-Eq-for-Timing\" class=\"impl\"><a href=\"#impl-Eq-for-Timing\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for Timing</h3></section>","Eq","teensy4_bsp::clock_power::Lpi2cBaud"],["<section id=\"impl-StructuralPartialEq-for-Timing\" class=\"impl\"><a href=\"#impl-StructuralPartialEq-for-Timing\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html\" title=\"trait core::marker::StructuralPartialEq\">StructuralPartialEq</a> for Timing</h3></section>","StructuralPartialEq","teensy4_bsp::clock_power::Lpi2cBaud"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()