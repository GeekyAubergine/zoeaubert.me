---
title: An Adventure Into Diffing Algorithms
date: 2016-11-01
tags:
categories:
- post
---
A while ago at work we decided that our current diffing algorithm was not
meeting our requirements. At the time we were using
[FineDiff](http://www.raymondhill.net/finediff/viewdiff-ex.php), while this
library is good at dealing with common situations, it was unable to handle
some of our more complicated inputs. The main complications we faced were
our use of `;` to separate items and the addition and removal of line breaks
without the meaning of the content changing.
<!--- more --->
Input A:
Alpha

Input B:
Alpha; Beta

Original output:
~~Alpha~~_Alpha; Beta_

Desired output:
Alpha; _Beta_

In this example the input has had the element `Beta` added to it, this in
turn has caused a separator to be included which is attachted to the word
`Alpha`. Most diffing algorithms subsiquently identify this as a 'new' word
and perform a diffing operation against it, which in this case produces a
confusing output.

Input A:
Alpha; Beta

Input B:
Alpha;
Beta

Original output:
Alpha; ~~Beta~~
_Beta_

Desired output:
Alpha;
Beta

In this case the addition of the line break between `Alpha` and `Beta`
causes it to identify it as a new word and complicates the output.

To resolve these issues I began to investigate how diffing algorithms work,
I found an [article](https://epxx.co/artigos/diff_en.html) on diffing
algorithms that turned out to be immensely useful. Before finding this
article I had experimented with a few different solutions and this article
pointed out where I had gone wrong and how to improve on it.

Using this I based my solution upon the longest common subsequence technique,