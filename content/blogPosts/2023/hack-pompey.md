---
slug: hack-pompey-2023
date: 2023-03-25T22:30
title: Hack Pompey 2023 - Blahbarian
description: I attended my first hackathon, and we made a game about a Norwegian barbarian shark who destroys furniture using TypeScript and canvas.
tags: ['Programming', 'GameDev', 'Hackathon']
hero: "https://cdn.geekyaubergine.com/2023/04572345cced.png"
heroAlt: "Two pixel art blue sharks wearing a barbarian horned had and a white shield with blue cross and red edging"
heroWidth: 256
heroHeight: 128
---

![Two pixel art blue sharks wearing a barbarian horned had and a white shield with blue cross and red edging](https://cdn.geekyaubergine.com/2023/04572345cced.png)

I attended my first hackathon hosted by [Hack Pompey](https://hackpompey.co.uk/), and it went well. 

tldr: We made a [Vampire Survivors](https://store.steampowered.com/app/1794680/Vampire_Survivors/) style game based around Blahai the Blahbarian

You can [play it here](https://zoeaubert.me/projects/blahbarian-hackathon/). Warning that the noises are pretty loud.

The code is available on [Github](https://github.com/GeekyAubergine/blahbarian/tree/hackathon).

## The Team

- [Char](https://github.com/bl-kt) - Blahbarian and weapon pixel art
- [Charlie](https://social.lol/@tldrqwerty) - Programming
- [Emily](https://emilymedhurst.me/) - Programming
- [Finn](https://github.com/PeacefulAndTranquil) - Enemy pixel art
- [Luke](https://github.com/LukeAustin8) - Programming
- [Robb](https://robbknight.me) (Remote) - Sound
- Me - Programming

## The Plan

Calling it a plan might be a bit of a stretch. My colleagues and I decided to team up when we decided to all go to the hackathon. We didn't know what we wanted to do and were hoping to find some inspiration when we got there. In the office, we have a [Blahaj](https://www.ikea.com/gb/en/p/blahaj-soft-toy-shark-30373588/), and it's been the centre of a few shenanigans. In a moment of inspiration, I came up with the idea of:

> Blahaj tower defence. The shark shoots boxes at furniture to flat-pack it.

This got everyone's attention. We quickly transitioned away from Blahaj and decided to move to Blahai, the legally distinct shark. We discussed the game style, and after floating over a few, including a platformer, we decided to go with a [Legend of Zelda](https://en.wikipedia.org/wiki/The_Legend_of_Zelda_(video_game)) style dungeon/room-based game.

To play into the joke some more, we decided to make the Blahai Norwegian and make him a barbarian who destroys furniture. Emily then suggested "Blahbarian", and we immediately knew this was what we must do.

## The Day

A rough outline of the day was:

- 10:15 - 13:00 - Programming
- 13:00 - 13:45 - Lunch
- 13:45 - 16:00 - Programming
- 16:00 - 17:00 - Presentations

We were likely to get a little under 5 hours of programming in. This was much shorter than I anticipated and was barely enough to finish.

As is standard, we were delayed at the start with internet issues, which caused us to lose about 15 mins at the start trying to get the repo cloned.

Once we got that going, we got [Vite](https://vitejs.dev/) installed for easy Typescript compiling and some other nice tooling. This then allowed me to quickly get a framework set up and rendering to the canvas for the others to work with. As I was building the framework, Charlie suggested we move to an even simpler game design of something more similar to Vampire Survivor (linked above). We all agreed this would be much simpler, and it turned out to be an excellent suggestion.

By lunchtime, we had the world, player and entities rendering, and the player could move around. The sprite sheet rendering was the first two frames of the walking animation.

After lunch, it was a bit of a sprint as we knew just how much we had left to do. We finalised how the sprite sheets worked to allow for animations and got them working directionally. Added spawning for enemies and basic "go to player" "AI". Added sounds. Added powerups. And finally, just before the presentation, we added attacking and attack animations.

The presentation went well, it didn't immediately fail, and we got through it in one piece. We came 3rd place in the community vote.

Overall it was a great experience. It wasn't stressful per se, but very draining. I look forward to doing it again and maybe participating in a game jam in the future.

## Engine design

Having dabbled in game building before, I had a plan of how it would work, which saved us a bunch of time. The core of the engine is simple.

The rendering would be handled by canvas. As the game was taking place on a flat plane, it made sense to centre the camera on the player and render everything relative to that. For the floor, this required rendering a simple grid of tiles relative to the player and ensuring the grid was one tile wider than the viewport. 

All entities were given a movement enum of 'IDLE', 'DOWN', 'UP', 'LEFT', and 'RIGHT' to allow us to render appropriately rotated sprites and then rendered on the canvas relative to the player. Animations were handled by looping through an array of sprite keys that referenced the sprite sheets and rendering the sprite. The animations were then tied to the movement enum to allow it to swap out the correct sprites depending on movement.

Damage dealing and collisions were handled by checking collisions on axis-aligned bounding boxes.

## What I'd do differently

Overall there's not a lot I'd do differently. The only part of the code I'm unhappy with is the animation system, as it's hard to trigger an animation and have it complete without a silly hack. For example, the sword swing takes longer than the player is pressing the key down for, so the boolean controlling its rendering is on a timeout rather than waiting for the last frame. It also doesn't always start from frame 0. I'm unsure how I'd address this without building a much more complex system than the time would've allowed.