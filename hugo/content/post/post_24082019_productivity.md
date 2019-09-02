+++
title = "Android, incremental builds and compilation times"
date = "2019-08-24"
author = "Gonçalo"
draft = false
+++

Productivity is all about minimizing bullshit.

I feel that I'm constantly complaining about compilation times.
Just create a new, empty Android project and see how much time it takes to have an application running in the device. It's ridiculous.

I'm far from being the only one complaining, it's 2019, we have faster computers than ever.

When Android's Jack and Jill project was announced, I had a little bit of hope that things would get better. Unfortunately, the project was dropped.

Providing compatibility with the existing plugin and library environment, proved to be too much. The fact that Kotlin might have been in the horizon of being officially supported probably didn't help.
It felt like it was time to throw everything into the thrash and start over. Sadly reality strikes you just have to go with what works.
I don't have any way to know if it would have solved my problems but it's the only thing I can grab onto :).

The consequence it's that now we're pretty much stuck with compiling and dexing and whatever else made sense in 2008.
It's feels like my computer is doing duplicate work every time I start to compile. When I see all the layers the build system has to go through, I have a hard time imagining that things will be improved significantly just from updates to the SDK and build tools. Especially if you have an existing big project.

If you're not careful with the dependencies you introduce to your project you'll see your compilation times explode. Just imagine one day waking up, and your project takes several minutes to compile.

Just as an aside: KAPT did not have incremental compilation up to just recently. KAPT will ruin your compilation times if you're not careful. Like KAPT there's more things like this.

I'm pretty much on the side that incremental compilation is just not ideal.
From my experience, problems with caches will ruin your day frequently.

Just to finish this rant, if you're starting a new programming language that you intend people use, please consider these things:
 - Computers can be really fast and you can have really fast compilation times.
 - Incremental builds might cause more problems than they fix on average.
 - Take compilation times as a feature of the language. Take also in consideration that many people will have an absurd amount of code to compile someday. It's not just about those hello world programs.
 - If you make it fast for a lot of code, chances are that it will be really fast for regular amounts of code.

That's it for today.
