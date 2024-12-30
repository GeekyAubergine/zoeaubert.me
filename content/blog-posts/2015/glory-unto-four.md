---
slug: glory-unto-four
title: Glory Unto Four
date: 2015-01-25
description: I made a meme programming language and it was terrible
tags: ["Programming", "Languages"]
---

## Disclaimer/Errata

This post was retrieved from the archives of an old website. I wrote this just after Christmas during my first year of University, as such it wasn't it's best. I've left it as true to the origial as possible, though I have cleared up some aggregious errors.

Unlike the original, I have also included the (terrible) source code (see bottom). No I don't know why I chose Java. Hopefully you'll find it a least a bit amusing.

---

I have created something both glorious and terrible, and I have called it Four.

What is Four? That's a good question. Four came about as a result of me showing a friend [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck). This spawned an idea, what if we could make something more difficult, more convoluted, thus Four was born. At the time we happened to have also been discussing 4chan (I won't link you, don't worry), and the idea was put forward to create a language only using the letter 'b', to pay homage to /b on 4chan (if you don't already know what that is, ask a friend, I would advise against looking it up personally). It's probably occurred to you that using a single character would be essentially impossible, so the choice was made to use '/' instead of space as a separator, why, because it was kinder than using 'd' and tied in very nicely with the theme we were going for.

Four is a rather interesting language, made difficult not only by it's reduced command set, but also the difficulties that occur when trying to read lots of b's and accurately count them (I know, I've been there). When designing Four I settled upon 15 commands (slightly more then Brainfuck's 8), as I felt this was the best combination of utility and 'simplicity'. It also behaves in a similar way to Brainfuck in the sense that it has a 'infinite' set of values in a single dimensional array. The instructions are as follows (the row number corresponds to the number of b's:

1.  Adds one to the current index
2.  Subtracts one to the current index
3.  Moves the current index right one position
4.  Moves the current index left one position
5.  Stores the current value at the current position in memory
6.  Sets the current value at the current position to the value stored in memory
7.  Adds the values stored in memory to the current value at the current position
8.  Sets the value in memory to the current index
9.  Adds one to the current value stored in memory
10. Subtracts one from the current value stored in memory
11. Inverts the current value in the current index
12. Multiples the current value in the current position by 2
13. Divides the current value in the current position by 2
14. Starts a new loop
15. End of loop segment

Syntax:  
All commands must be separated by a '/'. That's it. So adding one and moving right one would be 'b/bbb/'.

Notes:

- And values stored in a negative index are not printed
- At the end of the program all values are printed out
- To print the values as numbers the first command should be 'b/'. To print the values as ASCII the first command should be 'bb/'

Here's 'Hello, World!' in Four

```text
bb/
#Set -1 13
bbbb/
b/b/b/b/b/b/b/b/b/b/b/b/b/
#Grab 13
bbbbb/
#Set 65 across array
bbbbbbbbbbbbbb/
bbb/
b/b/b/b/b/b/b/b/
bbbbbbbbbbbb/
bbbbbbbbbbbb/
bbbbbbbbbbbb/
b/
bbbbbbbbbbbbbbb/
#Set -2 32
bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/
b/b/b/b/b/b/b/b/
bbbbbbbbbbbb/
bbbbbbbbbbbb/
bbbbb/
#Move 0
bbb/bbb/
#Set 0 H
b/b/b/b/b/b/b/
#Set 1 e
bbb/
bbbbbbb/
b/b/b/b/
#Set 2 l
bbb/
bbbbbbb/
b/b/b/b/b/b/b/b/b/b/b/
#Set 3 l
bbb/
bbbbbbb/
b/b/b/b/b/b/b/b/b/b/b/
#Set 4 o
bbb/
bbbbbbb/
b/b/b/b/b/b/b/b/b/b/b/b/b/b/
#Set 5 ,
bbb/
bbbbbbbbbbbbb/
bbbbbbbbbbbbb/
bbbbbbb/
bb/bb/bb/bb/
#Set 6(space)
bbb/
bbbbbbbbbbbbb/
#Set 7 W
bbb/
bbbbbbb/
bb/bb/bb/bb/bb/bb/bb/bb/bb/bb/
#Set 8 o
bbb/
bbbbbbb/
b/b/b/b/b/b/b/b/b/b/b/b/b/b/
#Set 9 r
bbb/
bbbbbbb/
b/b/b/b/b/b/b/b/b/b/b/b/b/b/b/b/b/
#Set 10 l
bbb/
bbbbbbb/
b/b/b/b/b/b/b/b/b/b/b/
#Set 11 d
bbb/
bbbbbbb/
b/b/b
#Set 12 !
bbb/
bbbbbbbbbbbbb/
b/
```

This outputs 'Hello, World!'.

For all those interested here's the minimised version:

```text
bb/bbbb/b/b/b/b/b/b/b/b/b/b/b/b/b/bbbbb/bbbbbbbbbbbbbb/bbb/b/b/b/b/b/b/b/b/bbbbbbbbbbbb/bbbbbbbbbbbb/bbbbbbbbbbbb/b/bbbbbbbbbbbbbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/bbbb/b/b/b/b/b/b/b/b/bbbbbbbbbbbb/bbbbbbbbbbbb/bbbbb/bbb/bbb/b/b/b/b/b/b/b/bbb/bbbbbbb/b/b/b/b/bbb/bbbbbbb/b/b/b/b/b/b/b/b/b/b/b/bbb/bbbbbbb/b/b/b/b/b/b/b/b/b/b/b/bbb/bbbbbbb/b/b/b/b/b/b/b/b/b/b/b/b/b/b/bbb/bbbbbbbbbbbbb/bbbbbbbbbbbbb/bbbbbbb/bb/bb/bb/bb/bbb/bbbbbbbbbbbbb/bbb/bbbbbbb/bb/bb/bb/bb/bb/bb/bb/bb/bb/bb/bbb/bbbbbbb/b/b/b/b/b/b/b/b/b/b/b/b/b/b/bbb/bbbbbbb/b/b/b/b/b/b/b/b/b/b/b/b/b/b/b/b/b/bbb/bbbbbbb/b/b/b/b/b/b/b/b/b/b/b/bbb/bbbbbbb/b/b/b/bbb/bbbbbbbbbbbbb/b/
```

Currently the compiler for this project is unavailable and will be until I see a demand to make it shippable product.

Overall this has been a very interesting project, regardless of how impracticable it is, it certainly turns a few faces.

## The Code

```java
package com.four.lang;

import java.util.ArrayList;
import java.util.HashMap;

import com.geekyaubergine.geekyutil.io.FileUtils;
import com.geekyaubergine.geekyutil.math.MathUtil;

public class B_B {

    public static final int MAX_INDEX = 100;
    public static int type = 0;
    public static int position = 0;
    public static int acc = 0;
    public static String separator = "/";

    static HashMap<Integer, Integer> map = new HashMap<Integer, Integer>();

    public static int parse(String[] data, int index) {
        String command = data[index];
        if (command.equals("bbbbbbbbbbbbbb")) { //14
            int counter = acc;
            int end = index;
            for (int i = index + 1; i < data.length; i++) {
                if (data[i].equals("bbbbbbbbbbbbbbb")) { //15
                    end = i;
                }
            }
            for (int n = 0; n < counter; n++) {
                for (int j = index + 1; j < end; j++) {
                    j = parse(data, j);
                }
            }
            return end;
        }
        else if (command.equals("b")) {
            if (!map.containsKey(position)) {
                map.put(position, 1);
            } else {
                map.put(position, map.get(position) + 1);
            }
        }
        else if (command.equals("bb")) {
            if (!map.containsKey(position)) {
                map.put(position, -1);
            } else {
                map.put(position, map.get(position) - 1);
            }
        }
        else if (command.equals("bbb")) {
            position++;
        }
        else if (command.equals("bbbb")) {
            position--;
        }
        else if (command.equals("bbbbb")) { //5
            acc = map.get(position);
        }
        else if (command.equals("bbbbbb")) { //6
            map.put(position, acc);
        }
        else if (command.equals("bbbbbbb")) { //7
            if (!map.containsKey(position)) {
                map.put(position, acc);
            } else {
                map.put(position, acc + map.get(position));
            }
        }
        else if (command.equals("bbbbbbbb")) { //8
            acc = position;
        }
        else if (command.equals("bbbbbbbbb")) { //9
            acc++;
        }
        else if (command.equals("bbbbbbbbbb")) { //10
            acc--;
        }
        else if (command.equals("bbbbbbbbbbb")) { //11
            map.put(position, -map.get(position));
        }
        else if (command.equals("bbbbbbbbbbbb")) { //12
            map.put(position, map.get(position) * 2);
        }
        else if (command.equals("bbbbbbbbbbbbb")) { //13
            map.put(position, map.get(position) / 2);
        }

        return index;
    }

    public static void parseFile(ArrayList<String> lines) {
        if (lines == null) {
            return;
        }
        map = new HashMap<Integer, Integer>();
        map.put(position, 0);

        String dataRaw = "";
        for (String line : lines) {
            if (!line.startsWith("#")) {
                if (!line.endsWith(separator)) {
                    line += separator;
                }
                dataRaw += line;
            }
        }
        FileUtils.deleteFile("min.txt");
        FileUtils.addToFile("min.txt", dataRaw);
        String[] data = dataRaw.split(separator);

        /* Data type */
        String dataTypeDec = data[0];
        if (dataTypeDec.equals("b")) {
            type = 1;
        }
        else if (dataTypeDec.equals("bb")) {
            type = 2;
        }

        for (int i = 1; i < data.length; i++) {
            //System.out.println(i);
            i = parse(data, i);
            position = (int) MathUtil.clampFloat(position, -MAX_INDEX, MAX_INDEX);
        }
    }


    public static void print() {
        FileUtils.deleteFile("output.txt");
        String out1 = "";
        String out2 = "";
        for (int i = 0; i < MAX_INDEX; i++) {
            if (map.containsKey(i)) {
                int n = map.get(i);
                char c = (char) (n);
                out1 += n + " ";
                out2 += c;
            }
        }
        String out = "";
        if (type == 1) {
            out = out1;
        }
        else if (type == 2) {
            out = out2;
        }
        FileUtils.deleteFile("out.txt");
        FileUtils.addToFile("out.txt", out);
        System.out.println("Out:");
        System.out.println(out);
    }

    public static void runFile(String file) {
        parseFile(FileUtils.getFileStrings(file, false));
        print();
    }

    public static void main(String[] args) {
        //File file = new File("res/test.txt");
        runFile("script.txt");
    }

}
```
