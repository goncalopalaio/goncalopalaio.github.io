+++
title = "Android and avoiding toxic zones"
date = "2019-04-27"
author = "Gonçalo"
cover = "hello.jpg"
description = "View bindings and avoiding zones that will take productivity away."
+++

There are these zones that while developing something will take away your time and never give anything back.

In case of Android, we have a multitude of tools that are glued together that when one of them fails, you're in for a world of pain. I'm not even going to talk about specifics, if you're building something complex enough you will have an example.

Google is now invested in creating libraries for developers to use, but I'm left asking myself why there isn't there a more visible movement of people that create their own set of tools.
Sure there are examples of libraries that established themselves  as the one way to do thing X but it appears that there is this sentiment that creating tools is not your job and should only done by Google, Stripe, etc.

As I write this, I'm thinking about view binding. You can now call it as being in the third generation. First you had Butterknife, which I have no experience in. Databinding, which is awesome, provided that it works and you don't begin having non-sense errors. Viewbinding, that provides a limited subset of functionality that Databinding already has.

What's common in all of these (not sure about Butterknife) they utilize incremental compilation to have they code generated. What's my problem with this? Incremental compilation causes more problems that it solves. Compilation times are still miserable, and you always are having cache problems and having to clear them from time to time (you're lucky if you haven't). What I envision would be the solution? You write your code generation as a separate program, generate the code, commit the code to you repository and you only re-generate it when you change anything. No more issues with caches, annotation processors etc. Sure you have one more thing to maintain and it's non-standard, but at least you can change it to match your use case. There, one less thing that will burn your time. There are a lot of small programs like this that I imagine you could have.

All of that to say that what works for a particular use case, might cause a lot of frustration for you. There's that old saying that you should not reinvent the wheel, but there are different types of wheels, you can't roll around Mars with the wheels of your bicycle.