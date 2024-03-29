

<a id="@A_Root_Documentation_Template_0"></a>

# A Root Documentation Template


This document contains the description of multiple move scripts.

The script <code><a href="root_template_script3.md#0x1_yet_another">yet_another</a></code> is documented in its own file.

-  [Some Scripts](#@Some_Scripts_1)
    -  [Module `0x1::some`](#0x1_some)
        -  [Function `some`](#0x1_some_some)
-  [Other Scripts](#@Other_Scripts_2)
    -  [Module `0x1::other`](#0x1_other)
        -  [Function `other`](#0x1_other_other)
-  [Some other scripts from a module](#@Some_other_scripts_from_a_module_3)
    -  [Module `0x1::OneTypeOfScript`](#0x1_OneTypeOfScript)
        -  [Function `script1`](#0x1_OneTypeOfScript_script1)
        -  [Function `script2`](#0x1_OneTypeOfScript_script2)
    -  [Module `0x1::AnotherTypeOfScript`](#0x1_AnotherTypeOfScript)
        -  [Function `script3`](#0x1_AnotherTypeOfScript_script3)
        -  [Function `script4`](#0x1_AnotherTypeOfScript_script4)
-  [Index](#@Index_4)



<a id="@Some_Scripts_1"></a>

## Some Scripts



<a id="0x1_some"></a>

### Module `0x1::some`



<pre><code></code></pre>



<a id="0x1_some_some"></a>

#### Function `some`

This script does really nothing but just aborts.


<pre><code><b>fun</b> <a href="root.md#0x1_some">some</a>&lt;T&gt;(_account: signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="root.md#0x1_some">some</a>&lt;T&gt;(_account: signer) {
    <b>abort</b> 1
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>true</b> <b>with</b> 1;
</code></pre>



</details>



<a id="@Other_Scripts_2"></a>

## Other Scripts



<a id="0x1_other"></a>

### Module `0x1::other`



<pre><code></code></pre>



<a id="0x1_other_other"></a>

#### Function `other`

This script does also abort.


<pre><code><b>fun</b> <a href="root.md#0x1_other">other</a>&lt;T&gt;(_account: signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="root.md#0x1_other">other</a>&lt;T&gt;(_account: signer) {
    <b>abort</b> 2
}
</code></pre>



</details>

<details>
<summary>Specification</summary>



<pre><code><b>aborts_if</b> <b>true</b> <b>with</b> 2;
</code></pre>



</details>



<a id="@Some_other_scripts_from_a_module_3"></a>

## Some other scripts from a module



<a id="0x1_OneTypeOfScript"></a>

### Module `0x1::OneTypeOfScript`



<pre><code></code></pre>



<a id="0x1_OneTypeOfScript_script1"></a>

#### Function `script1`

This is a script


<pre><code>entry <b>fun</b> <a href="root.md#0x1_OneTypeOfScript_script1">script1</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>fun</b> <a href="root.md#0x1_OneTypeOfScript_script1">script1</a>() {}
</code></pre>



</details>

<a id="0x1_OneTypeOfScript_script2"></a>

#### Function `script2`

This is another script


<pre><code>entry <b>fun</b> <a href="root.md#0x1_OneTypeOfScript_script2">script2</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>fun</b> <a href="root.md#0x1_OneTypeOfScript_script2">script2</a>() {}
</code></pre>



</details>


This is another module full of script funs too:


<a id="0x1_AnotherTypeOfScript"></a>

### Module `0x1::AnotherTypeOfScript`



<pre><code></code></pre>



<a id="0x1_AnotherTypeOfScript_script3"></a>

#### Function `script3`

This is a script


<pre><code>entry <b>fun</b> <a href="root.md#0x1_AnotherTypeOfScript_script3">script3</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>fun</b> <a href="root.md#0x1_AnotherTypeOfScript_script3">script3</a>() {}
</code></pre>



</details>

<a id="0x1_AnotherTypeOfScript_script4"></a>

#### Function `script4`

This is another script


<pre><code>entry <b>fun</b> <a href="root.md#0x1_AnotherTypeOfScript_script4">script4</a>()
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>fun</b> <a href="root.md#0x1_AnotherTypeOfScript_script4">script4</a>() {}
</code></pre>



</details>



<a id="@Index_4"></a>

## Index


-  [`0x1::AnotherTypeOfScript`](root.md#0x1_AnotherTypeOfScript)
-  [`0x1::OneTypeOfScript`](root.md#0x1_OneTypeOfScript)
-  [`0x1::other`](root.md#0x1_other)
-  [`0x1::some`](root.md#0x1_some)
-  [`0x1::yet_another`](root_template_script3.md#0x1_yet_another)
