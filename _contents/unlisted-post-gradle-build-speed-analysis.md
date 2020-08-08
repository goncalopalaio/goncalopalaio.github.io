title = Android - Ways to tell your build is slow
date = 08-08-2020

#### Number 1 - Don't even bother

Just to set the expectations, your build is already slow. Two seconds to build an empty project? preposterous. Computers are fast, software should also be.

#### Number 2 - Use --profile

You can use --profile to get a basic overview of what's taking all of your precious time (https://developer.android.com/studio/build/optimize-your-build).

#### Number 3 - Use Gradle Build Scans

By using --scan in your build you can get a detailed overview of what tasks ran, which ones ran from cache and took most of the time (https://guides.gradle.org/performance/#build_scans)

#### Number 4 - Go deeper, use Gradle Profiler

The most interesting part of Gradle Profiler (https://github.com/gradle/gradle-profiler) is that you can create multiple scenarios in your project and measure those scenarios individually. You want to know what happens when you change a particular file in your project? Gradle Profiler got you covered. Gradle Profiler will produce a flamegraph or a detailed overview of what happened that you can check in your favorite profiler.

You now removed all of those tasks that were making the whole project re-compiled again? It's still slow?

You have multiple options, you can write a compiler that actually prioritizes speed for debug builds or read the first section in this page (sorry for the snarkiness).

Well, what now?
Is javac taking all of your time? Not sure if it will help but you can look into the next section.

#### Number 5 - Go even deeper, write a compiler plugin

Do you have a lot of files or do you have a lot of code? Both?
One way of checking what's happening in javac is writing a custom plugin. I think you can do the same for kotlinc but haven't checked in detail.

TODO - Actually do the research. How to write it. Is there any measurements about where the time is taken and what could be changed?