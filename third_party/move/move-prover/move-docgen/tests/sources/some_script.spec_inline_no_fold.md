
<a id="0x1_some"></a>

# Module `0x1::some`



-  [Function `some`](#0x1_some_some)


<pre><code></code></pre>



<a id="0x1_some_some"></a>

## Function `some`

This script does really nothing but just aborts.


<pre><code><b>fun</b> <a href="some_script.md#0x1_some">some</a>&lt;T&gt;(_account: signer)
</code></pre>



##### Implementation


<pre><code><b>fun</b> <a href="some_script.md#0x1_some">some</a>&lt;T&gt;(_account: signer) {
    <b>abort</b> 1
}
</code></pre>
