---
slug: hackathon-accelos
date: 2023-11-19T23:30
title: Hackathon - AccelOS
description: I attended a hackathon with the theme Silly Interfaces and created a balance-based input for a simple operating system.
tags: ['Programming', 'GameDev', 'Hackathon']
hero: "https://cdn.geekyaubergine.com/2023/11/19/2023-11-19T22.55.24.png"
heroAlt: "Screen shot of a basic operating system interface listing Main, Files and Email on the left and side. On the right hand side a maze with various coloured circles and walls"
heroWidth: 980
heroHeight: 449
---

![Screen shot of a basic operating system interface listing Main, Files and Email on the left and side. On the right hand side a maze with various coloured circles and walls](https://cdn.geekyaubergine.com/2023/11/19/2023-11-19T22.55.24.png)

Yesterday, I attended a hackathon with the theme "Silly Interfaces". 

TLDR: I created a game where you had to balance a blob through a maze, changing its colour to match the colour of the option you want to select from an Operating System (OS) menu. The game is played on the browser, but the balance controls are controlled by tilting your phone using a companion app (not included). 

You can [play the game](https://accelos.zoeaubert.me/) and [read the code](https://github.com/GeekyAubergine/accelo-os). To make it playable without a mobile device, I have changed the balance input to a keyboard input using WASD. To change the blob's colour, go over one of the coloured circles (you can go over multiple to combine colours to make the other colours). To select an option, you must go over the coloured square of the option you want, and your blob must be that colour.

Why's it called AccelOS? Because Accelerometer OS. And yes, I realise now, after calling it that, that actually I'm using the gyroscope, not the accelerometer, but here we are 🤷‍♀️.

## The Idea

After a short discussion with others about what constituted "silly", I was left with an initial choice. Did I want to go with "silly" as in annoying or "silly" as in comical? I was heading down the comical route until I overheard another conversation about using facial expressions to control an OS. They quickly dismissed the idea, but I liked it, though I didn't want to do anything with machine learning. So, I was left with the challenge of coming up with a silly way to control an OS.

A long time ago, I spent a silly number of hours playing [Mercury Meltdown](https://en.wikipedia.org/wiki/Mercury_Meltdown) on the [PSP](https://en.wikipedia.org/wiki/PlayStation_Portable). In it, you have to move blobs of mercury around a tiltable map and change the colour of the blobs to match the goal colours. I loved this game and how complex the controls were (I've often thought about remaking it). I decided this was probably a good input system. You move and change the colour of a blob around to make it the colour of the option you want to select. I was initially going to include some of the more complex mechanics of Mercury Meltdown, such as splitting and merging blobs, but that proved too complex.

To top this off, rather than just your keyboard, imagine if you had to balance the map using your phone. Not only was this annoying because it's very hard to hold it perfectly level, but because the game is on one screen and your phone is in your hand. There's a weird disconnect and mental lag compared to it all being on your phone, which adds to the fun.

## The Build

I turned to familiar tools for this. Similar to my [previous hackathon](https://zoeaubert.me/blog/hack-pompey-2023/), I decided to use TypeScript (TS) with the canvas and [Vite](https://vitejs.dev/). Borrowing some [code](https://github.com/GeekyAubergine/blahbarian/tree/hackathon) and styles from that project, I was quickly up and running.

The first step was to get the core game functionality going. I decided on a much less complex system for rendering than previously, which helped a lot, though it has some rough edges. I wouldn't do this in a team, but it was just fine for solo. After getting the map and blobs rendering, it was time to work on the physics. This was pretty straightforward, to the point that there are bugs; for example, the blob does a simple collision check on the top, bottom, left and right of it before moving and stops moving in that direction. You can slightly clip inside if you approach a wall at an angle. It's annoying but not problematic enough to spend too much time on it.

From there, setting up the blob colour mechanics and goal detection wasn't too much hassle. I initially made the colour system very flexible, but after some complications, I decided to make it a small enum of just the colours you see. Adding more is very manual, so I didn't go any further.

Flushed with confidence, I decided to build the map. This took a surprisingly long time. In my infinite wisdom, I built the map manually in code rather than parsing a map file. It might've been quicker in retrospect to build the parser, but by the time I came to that conclusion, it was too late. 

This then leads me to what is a significant part of the project. The phone balancing system.

### Mobile Shenanigans

I remember learning some time ago that the browser had access to the [gyroscope](https://developer.mozilla.org/en-US/docs/Web/API/Gyroscope), and I was right, except I very much wasn't.

The problems first started when for some reason, no mobile device could access the site. It ran fine on people's laptops, but no mobile device could resolve the host. I wrote this off to some oddity with the network at the venue or something else. I was thankfully able to connect to it from my phone if I was on a mobile hotspot though, so I used that. 

Thinking that was the end of my problems, I charged on and added the code to access the gyroscope. Enter roadblock 2. Chrome does not actually support the gyroscope, and neither does Safari (or mobile). I then checked the [caniuse](https://caniuse.com/gyroscope) for it and, with relief, noticed that Android browsers seemed to support it. So, I borrowed an Android device. Unsurprisingly, despite it being up to date, it didn't work either.

Now entering a mild panic, I turned to the only tool I knew _might_ work, [ReactNative](https://reactnative.dev/). I quickly built what is probably my most janky project to date. The code for it is not Github as it is not worthy of it, and I don't want people thinking it's good. But you can enjoy it here.

```ts
import React, {useEffect} from 'react';
import {
  Button,
  SafeAreaView,
  StatusBar,
  Text,
  useColorScheme,
} from 'react-native';

import {
  gyroscope,
  setUpdateIntervalForType,
  SensorTypes,
} from 'react-native-sensors';
import {Colors} from 'react-native/Libraries/NewAppScreen';

let gyro = {x: 0, y: 0, z: 0};

function App(): JSX.Element {
  const isDarkMode = useColorScheme() === 'dark';

  useEffect(() => {
    const ws = new WebSocket('ws://172.20.10.3:3000');
    console.log('connecting');
    ws.onopen = () => {
      console.log('connected');

      setInterval(() => {
        ws.send(
          JSON.stringify({
            type: 'gyro',
            ...gyro,
          }),
        );
        // console.log('sent');
      }, 100);
    };
  }, []);

  const backgroundStyle = {
    backgroundColor: isDarkMode ? Colors.darker : Colors.lighter,
  };

  setUpdateIntervalForType(SensorTypes.gyroscope, 20); // defaults to 100ms

  gyroscope.subscribe(({x, y, z}) => {
    gyro = {
      x: x + gyro.x,
      y: y + gyro.y,
      z: z + gyro.z,
    };
  });

  return (
    <SafeAreaView style={backgroundStyle}>
      <StatusBar
        barStyle={isDarkMode ? 'light-content' : 'dark-content'}
        backgroundColor={backgroundStyle.backgroundColor}
      />
      <Text>Gyroscope:</Text>
      <Button
        onPress={() => {
          gyro = {x: 0, y: 0, z: 0};
        }}
        title="Reset"
      />
    </SafeAreaView>
  );
}

export default App;

```

This took a while to get working, but thank your deity of choice, it did. 

One problem I almost encountered was Apple's developer licensing. If I hadn't had access to the Android device, I wouldn't have been able to test it on any device, as I was not paying for the license to build it on my phone. 

Thankfully, this worked as intended, and after some playing around, I integrated it with the website. One fun thing I didn't notice in the documentation is that this library only sends delta updates, so you have to track the communicative value to get the actual tilt of the device.

### The OS Part

At this point, I was running out of steam. I opted for a very quick and dirty implementation of the "OS". It's a series of states with a list of options. The game part dispatches a "player triggered goal with colour X" event, and the OS part determines if it's a valid option and transitions to the next suitable state. It's very basic, but is enough to demonstrate the silly part which is the control scheme.

## Conclusion

This was a lot of fun. I don't consider myself much of a "creative ideas" person and usually rely on others in the team to come up with ideas and implement them. But as I was solo this time, so I couldn't do that, and I was very pleased with what I came up with, even if part of the idea was "borrowed".

There wouldn't be much I'd do differently other than not do something that interacts with the core components of a mobile device unless the whole project is on mobile. And if it is, it would be the first thing I'd check to see if it worked. While the project is doable with keyboard input, it lacks a lot of charm that the balancing aspect introduces. Without it, I'd've felt bad about presenting it. So, leaving it so late to test it was silly.

Overall, I consider this a successful project.