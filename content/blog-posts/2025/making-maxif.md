---
date: 2025-02-02T10:05:00
title: Making MAXIF
slug: making-maxif
description: I made a tool for inspecting file signatures and other metadata
tags: ["Programming"]
---

Files are difficult to work with. While file extensions and [MIME](https://en.wikipedia.org/wiki/MIME) types are often correct, sometimes they are not.

I've had this happen many times with the various systems I've worked on. Someone uploads a file that should work, but it is rejected. And often, it's not immediately apparent why it was rejected.

My next step is to open the file in a [Hex Editor](https://en.wikipedia.org/wiki/Hex_editor), inspect the bytes, and compare them to a list of [File Signatures](https://en.wikipedia.org/wiki/File_format#Magic_number). More often than not, the file extension is wrong, and the file is of a different type that the system doesn't accept.

This process is slow, fault-prone, and annoying. This is only made harder with some file signatures not being easy to validate by eye.

After recently having it happen a few times in short succession, I decided I should probably solve the problem properly.

Introducing [MAXIF](https://maxif.zoeaubert.me). A simple tool for inspecting a file and auto-detecting file signatures.

![Screen shot of MAXIF being used. At the top of the image is the heading "File Inspector" with the sub-text "Upload a file to inspect its contents and metadata". The rest of the UI is split into three sections. On the left is a file uploader. The middle shows a list of file data, in including file name ("file_example_WEBP-1500kB.webp"), type (WebP), file signature (52 49 46 46 ?? ?? ?? ?? 57 45 42 50), signature offset (0), size (1.42MiB or 1490956 bytes). The right-hand section shows a hexadecimal preview of the file's contents with several bytes highlighted in blue.](https://cdn.geekyaubergine.com/2025/02/making-maxif/maxif-usage-example.png)

## Tech Bit

My goals for MAXIF were:
- Identify and show the file signature of an uploaded file
- Not require the file to be uploaded anywhere
- Show metadata about the file
- Stretch goal: Parse [EXIF](https://en.wikipedia.org/wiki/Exif) and other metadata from the files

To do this, I turned to [Vite](https://vite.dev/) and [React](https://react.dev/). Some would argue that these are overkill, and they'd probably be right. I mostly used Vite because it handles a lot of the boilerplate that comes with wanting to use [TypeScript](https://www.typescriptlang.org/) and the like.

After getting that all set up, the next goal was handling files. This proved a lot easier than I expected. I ended up using the basic built-in [FileReader](https://developer.mozilla.org/en-US/docs/Web/API/FileReader), [Uint8Array](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Uint8Array) and [DataView](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/DataView). DataView was particularly useful as it handles [Endianness](https://en.wikipedia.org/wiki/Endianness) for you.

I did wrap up most of this functionality into my own `FileReader`- like interface. This habit, which I've picked up over the years, proved useful, as I was able to change several aspects of the implementation without having to update every other file.

After successfully reading the file as bytes, I turned to parsing and matching the file signatures. Since I was using the [Wiki File Signatures list](https://en.wikipedia.org/wiki/List_of_file_signatures),  I wanted to support the ability for me to copy and paste from that list, so my matcher had to accept their format.

```typescript
public constructor(
  signatureAsString: string,
  offset = 0,
  name: string,
  description: string | null = null,
) {
  this.signatureAsString = signatureAsString;
  this.offset = offset;
  this.name = name;
  this.description = description;

  const s = signatureAsString.replace(/\s/g, "");

  this.fileSignature = [];

  for (let i = 0; i < s.length; i += 2) {
    const sub = s.substring(i, i + 2);

    if (sub.length === 0) {
      break;
    }

    if (sub === "??") {
	  this.fileSignature.push({ type: "wildcard" });
      continue;
    }

    if (sub.length !== 2) {
      throw new Error(
       `Invalid signature format: ${signatureAsString}. Next byte at ${i} is not 2 characters long [${sub}]`,
      );
    }

    this.fileSignature.push({ type: "number", number: parseInt(sub, 16) });
  }

  this.signatureAsUint8Array = new Uint8Array(
    this.fileSignature.map((v) => {
      if (v.type === "wildcard") {
        return 0
      }

      return v.number;
     }),
   );

  FileSignature.FILE_SIGNATURES.push(this);
}
```

This signature can then be used to compare to an [ArrayBuffer](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) and return a match or not.

```typescript
public matches(buffer: ArrayBuffer): FileSignatureMatch | false {
  const data = new Uint8Array(
    buffer.slice(this.offset, this.offset + this.fileSignature.length),
  );

  const relevantBytes: number[] = [];

  for (let i = 0; i < this.fileSignature.length; i += 1) {
    const magicNumber = this.fileSignature[i];

    if (magicNumber.type === "wildcard") {
      continue;
    }

    const byte = data[i];

    if (byte !== magicNumber.number) {
      return false;
    }

    relevantBytes.push(i + this.offset);
  }

  return {
    name: this.name,
    relevantBytes,
    signatureAsString: this.signatureAsString,
    signatureOffset: this.offset,
  };
}
```

After that, most of the product was done. The rest involved fixing the UI to make it look good and adding all the file signatures.

I have also laid the groundwork for parsing the file to extract more data; this can be seen in the [WebP parser](https://github.com/GeekyAubergine/maxif/blob/main/src/parser/ParserWebP.ts)

Other than that, it's not very interesting. I'm annoyed I didn't make it earlier, considering how useful it is.

## Closing Thoughts

I recently adopted the mantra "Finish Things"; this was my first project under that new thinking.

Despite some efforts to get distracted with trying to make Rust work for the project, I stayed on task and got it deployed.

Granted, I haven't yet parsed all the EXIF and metadata. But that wasn't part of the original MVP. I should slowly chip away at it rather than delay the whole project.

Overall, I consider this a success. I finished it in a reasonable amount of time, and it has already proved useful.
