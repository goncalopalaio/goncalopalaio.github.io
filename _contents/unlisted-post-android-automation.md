title = Ways to automate interactions in Android
date = 15-08-2020

Here's a list of some of the ways you have to automate interactions in your android device or android application.

I will mostly ignore the fact that you could add uiautomator or espresso to your project and perform your actions there. What I am interested is in solutions where it's not required that you make changes to your project.

I will not go into detail about most of the command arguments, but in almost all cases they are pretty obvious.

# 1 - Input with ADB

`adb shell input`

ADB by itself works pretty well but has a few inconveniences.
I will list some of them in each section, but in general these commands are pretty slow to execute so they might suit your use case if you need to perform the actions quickly.

These are the most common commands but there is also **press** and **roll**

## Text

`adb shell input text "Some\ Text!"`

*What it does*: Writes the text in the focused text input like it was being written in the keyboard, character by character.

*Cons*: You have to escape the spaces and if you need to paste some weird characters like emoji it might not be supported.

## Tap

`adb shell input tap 394 2098`

*What it does*: Performs a touchscreen tap in a specific screen coordinate.

*Cons*: You have to fumble around into getting the correct coordinates to do what you want.

## Swipe

`adb shell input swipe 540 1600 540 1800 200`

*What it does*: Performs a touchscreen swipe from a screen coordinate into another with a set duration.

*Cons*: Has the same as **input** but now you also have to consider the swipe duration (last argument).

## Key events

`adb shell input keyevent 66`

*What it does*: Android devices have special keys, for example the home button or the volume up and down buttons. This command triggers an event for that key as if they pushed by the user.

*Cons*: You will have to search for the integer that correspond to they key. You can solve this by creating alias for the values in this file: https://cs.android.com/android/platform/superproject/+/master:bionic/libc/kernel/uapi/linux/input-event-codes.h

## Others
You can look into the remaining commands by typing:

`adb shell input`

At this point the remaining are **press** and **roll** which performs actions over the trackball that Android devices used to have.

# Making adb shell input a little bit better

My biggest issue with adb shell input is that I always have to out of my way to find the screen coordinates by enabling the setting in the developer options that shows the coordinates on the screen.

For this you can either use ==TODO adb automator command== to look where the view you're interested in is, or look at the screen coordinates through adb shell getevent. In case of getevent the values are given in hexadecimal so you'll have to do that conversion first.

I've created two scripts for both cases. 

- get-view-centers will give you the screen coordinates of the view id or view text that you provide as an argument to the script.
- ==TODO script name== will print the screencordinates as an integer. There's an alternative mode, but more on that later.

## Replicating events through getevent and sendevent

There's a lot of scattered information on this throughout the internet. getevent will list all the events that the device sensors are receiving and send event allows you to send some of them. 

There are three limitations by default that prevent you simply saving what getevent gives you and piping it directly to sendevent.
First of all, getevent will not provide you timing events ==TODO confirmation needed== so you know the user touched the screen, but not at what point after you started recording the events. Second, getevent uses a different format than sendevent so you will have to convert the events first.

You can look at the hex representation of getevent events at ==TODO link to file== 

==TODO example on how to translate the events==

The third limitation is that sendevent is slow to send events as it looks like it was never meant to be used to send multiple events.
It opens the file descriptor of the device sensor, sends a single event and then closes the file descriptor ==TODO Link to source==

To get around this, some smart folks already had the idea of recompiling sendevent so it would receive several events so they could be written in one shot to the device sensor file descriptor.

So here's the thing, you cannot have executables in the internal storage of the device. It makes sense you wouldn't be able to, but how did they get around this? apparently you're allowed to push to ==TODO temp path== and then call the executable with adb (not from the device itself).

You push the executable and then run:

adb shell mysendevent ==TODO replace with actual command==

These are the two places where I saw this being done:

==TODO links to mysendevent repos==

# ==TODO python monkey==

# Automation suites in the device

## Creating a accessibility service

If you're annoyed that you always have to write the exactly same text in the exact same place, you could create an accessibility service just for this. I know it's kind of abusing the original intent of the tool, but it works.
While active, it will continuously run and look and do what you want.
Keep in mind that this will go through all of your views attributes and might even reveal some problems dealing with accessibility services. That might even be a bonus.

==TODO link to example project==


## Using UiAutomator

This is the most flexible of the previous options. You pretty much have control of all of your device. The only downside over the accessibilty service is that you have to start the actions explicitly. 

This is normally used in a project so you can do interactions outside of your application but you can also take advantage of this and make it a generic way of performing any action at any time.

For this you have to create a separate project and run your tests which will install a special test apk. This creates a special entry point for the test that you wrote and allows you to start a specific test:

==TODO command to initiate tests==


### Make the tests dynamic

Instead of having a particular set of actions encoded in your tests. You can also make it a little more dynamic.

You can pass custom parameters to the test. With this you can pretty much do whatever you want. For example you can have a single test and according to the parameters perform different actions.

==TODO link to a small example project==


You could even embed a small language interpreter, for example LUA, add some bindings to the uiautomator methods, send a program as a parameter to the test and have it run within the test. I've started a similar project but it's not ready for prime time.

The extreme extension of this idea is:

==TODO link to uiautomator python thing==

Which as far as I know, starts a test and blocks it forever in order to have a server running continuously that sends and receives commands from an external python program in a RPC kind of way.
