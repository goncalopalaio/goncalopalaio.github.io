title = Actually profiling a gradle build on Android
date = 21-06-2020

## Why write about this again?

This is mostly common knowledge but I feel that most of the written blog posts are about gradle build scans which at some level are useful but depending on the bottlenecks might not say enough.

I will skip gradle build scans since it's pretty well documented and easy to use.

## First part - Gradle Profiler

Since gradle heavily relies on caching to make java/kotlin compilation faster there are several scenarios that influence how fast you can build.

Every post about gradle performance will tell you to avoid unnecessary changes before a build, for example if you have the current time in your build version, re-compilation of certain parts of your project might happen.

Gradle Profiler https://github.com/gradle/gradle-profiler has a way to automatically test and compare several scenarios where a particular file has changed. Adding a public method might have a different effect than a change in the body of a method. Changes to AndroidManifest.xml or any other resource might have other.

You can create gradle-profiler.scenarios and define several scenarios:

    scenario_change_to_android_manifest {
        title = "Changing Android Manifest"
        tasks = ["assembleDebugOrYourParticularTask"]
        apply-android-manifest-change-to = "AndroidManifest.xml"
        clear-build-cache-before = SCENARIO
        clear-transform-cache-before = BUILD
        show-build-cache-size = true
    }

    scenario_abi_change_to_file {
        title = "Adding a public method"
        tasks = ["assembleDebugOrYourParticularTask"]
        apply-abi-change-to = "src/com/yourpackage/MainActivity.java"
        clear-build-cache-before = SCENARIO
        clear-transform-cache-before = BUILD
        show-build-cache-size = true
    }

    scenario_non_abi_change_to_file {
        title = "Change to method body"
        tasks = ["assembleDebugOrYourParticularTask"]
        apply-non-abi-change-to = "src/com/yourpackage/MainActivity.java"
        clear-build-cache-before = SCENARIO
        clear-transform-cache-before = BUILD
        show-build-cache-size = true
    }

    scenario_change_to_strings {
        title = "Change to strings"
        tasks = ["assembleDebugOrYourParticularTask"]
        apply-android-resource-change-to = "res/values/strings.xml"
        clear-build-cache-before = SCENARIO
        clear-transform-cache-before = BUILD
        show-build-cache-size = true
    }

You can use it the following way (assuming your gradle project is in the current working directory):

    PROFILER_BIN=gradle-profiler/build/install/gradle-profiler/bin/gradle-profiler
    GRADLE_USER_HOME=gradle-profiler-gradle-user-home/
    $PROFILER_BIN --benchmark --project-dir . --profile jprofiler --output-dir $output_directory --gradle-user-home $GRADLE_USER_HOME --scenario-file gradle-profiler.scenarios --warmups 2 --iterations 4    

Note that you can use multiple profilers (more details in the gradle profiler github site). Here jprofiler is being used but you have multiple options. To try it out first you can use chrome-trace which will generate an interactive html page altough you can get more details with other profiler formats. Jprofiler will give you a super detailed view of the call stack.

You will get both an html report comparing all scenarios and the respective profiler data.


####### TODO INSERT HTML SCREENSHOT HERE #######

####### TODO INSERT JPROFILER SCREENSHOT HERE #######

####### TODO INSERT CHROME TRACE SCREENSHOT HERE #######



## Second part - Javac compiler plugin


####### TODO INSERT HTML SCREENSHOT HERE #######