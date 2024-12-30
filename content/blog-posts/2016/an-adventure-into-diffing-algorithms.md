---
slug: an-adventure-into-diffing-algorithms
title: An Adventure Into Diffing Algorithms
date: 2016-11-01
description: A dive into the wild world of building a custom diffing algorithms
tags: ["Programming", "DiffingAlgorithms", "Algorithms"]
---

A while ago at work, we decided that our current diffing algorithm was not
meeting our requirements. At the time we were using
[FineDiff](http://www.raymondhill.net/finediff/viewdiff-ex.php), while this
library is good at dealing with common situations, it was unable to handle some
of our more complicated inputs. The main complications we faced were our use of
`;` to separate items and the addition and removal of line breaks without the
meaning of the content changing.

Input A:  
Alpha

Input B:  
Alpha; Beta

Original output:  
~~Alpha~~_Alpha; Beta_

Desired output:  
Alpha; _Beta_

In this case, the element `Beta` has been added to the input string, when doing
this our software added the `;` separator between the elements. This results in
the original `Alpha` being compared to `Alpha;` which results in the diffing
algorithm determining that they are different words and mark them as a complete
replacement.

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

In this case, the addition of the line break between `Alpha` and `Beta` causes
it to identify it as a new word and complicates the output (it should be noted
that some algorithms can be configured to ignore newlines).

The results from the longest common subsequence implementation provided marginal
improvements, with our usage of `;` as a separator character. To further improve
this I arrived at a solution of each input being tokenised using whitespace as
separators, I then take these tokens and process a copy of them removed the `;`
and `\n` characters, for example:

Input:  
`Alpha; Bravo, Charlie; Delta`

Raw Tokens:  
`Alpha;` `Bravo, Charlie;` `Delta`

Clean Tokens:  
`Alpha` `Bravo, Charlie` `Delta`

Upon doing this to each set of inputs the algorithm could be performed against
the clean tokens, and the resulting diff would produce the results we wanted. It
was then just a case of keeping a reference to the raw token and replacing the
cleaned token with it when reconstructing the output string. The output from the
algorithm performed significantly better than we had expected.

It became quickly apparent that while the new algorithm performed well, it was
prone to producing verbose outputs (this was true of the previous implementation
as well).

Input A: a c d f g i k

Input B: a b d e g h i j

Output: a _b_ ~~c~~ d _e_ ~~f~~ g _h_ i _j_ ~~k~~

As you can see this output is incredibly verbose and complicated and proved
unusable for many users. We decided that if there were a significant amount of
difference between the input and outputs, then it would be displayed as a before
and after text, in the last example this would produce:

~~a c d f g i k~~
_a b d e g h i j_

This additional was as difficult to implement reliably as the general diffing
algorithm as it relied upon determining if the percentage diff was above a given
threshold per line, and then amending the output as required. Firstly the output
of the diffing algorithm is rendered into HTML tags and then split by `\n`, each
line then have `line.replace(/<\w>.*?<\/\w>/sg, '')` applied to it to remove all
text between tags, this allowed us to determine how many characters difference
there was between the lines. If the calculated percentage was over the threshold
two new string would be created, the first using
`line.replace(/(<i>.*?<\/i>)|(<(s|\/s)>)/gs, '')` to remove all italics tags and
any text between strikethroughs, and the second with the inverse operation
`line.replace(/(<s>.*?<\/s>)|(<(i|\/i)>)/gs, ''`. This new output or the
original line is then appended to the final output of the algorithm.
