title = Ways to tell your Android build is slow
date = 08-08-2020

This is meant to be a general overview of what you can do to measure how slow is your android Gradle build. Just google around and you'll find a multitude of posts that will give you magical Gradle flags that are now probably deprecated and didn't do much in the first place.

Be aware that there's the possibility that this gets a little bit ranty.

So, in how many ways can I measure my build speed?

#### 1 - Don't even bother

Just to set the expectations, your build is already slow. More than two seconds to build an empty project is ridiculous.

#### 2 - Use --profile

You can use **--profile** to get a basic overview of what's taking all of your precious time (https://developer.android.com/studio/build/optimize-your-build).
It's a good start but pretty basic.

#### 3 - Use Gradle Build Scans

Using **--scan** in your build you can get a detailed overview of what tasks ran, which ones ran from cache and took most of the time. Check the details at https://guides.gradle.org/performance/#build_scans

#### 4 - Use Android Studio Build Analyzer

Since Android Studio 4.0 you can now have a pretty graph that tells you which tasks took the longest.

#### 5 - Go deeper. use Gradle Profiler

The most interesting part of Gradle Profiler (https://github.com/gradle/gradle-profiler) is that you can create multiple scenarios in your project and measure those scenarios individually. Do you want to know what happens when you change a particular file in your project? Gradle Profiler got you covered. Gradle Profiler will produce for all scenarios a detailed overview of what happened, especially if you use your favourite profiler to look into it.

You now removed all of those tasks that were making the whole project re-compiled again? It's still slow? The problem is with javac or kotlinc?
You have multiple options. You can scour the internet for magic Gradle flags, write a compiler that prioritizes speed for debug builds (ha!) or read the next section on this page.

#### 6 - Go even deeper. Write a compiler plugin

Do you have a lot of files or do you have a lot of code? Both?
One way of checking what's happening in javac is by writing a custom plugin. Certain callbacks that will tell you when a file started being compiled and when it finished. You should be able to do the same for kotlinc but haven't checked in detail.

It's perfectly reasonable to think that different language features will have different costs. Maybe you have too many files, maybe your 100 deep inheritance class tree is grinding the compiler to a halt. I haven't done proper research into this but I just want to leave this as an option if you're desperate.

In summary. It's a mess.

