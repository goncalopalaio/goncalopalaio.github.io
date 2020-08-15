title = Ways to automate interactions in Android
date = 15-08-2020

Here's a list of some of the ways you have to automate interactions in your android device or android application.

I will not go into detail about most of the command arguments, but in almost all cases they are pretty obvious.

## 1 - adb shell input

ADB by itself works pretty well but has a few inconveniences.
I will list some of them in each section, but in general these commands are pretty slow to execute so they might suit your use case if you need to perform the actions quickly.

These are the most common commands but there is also **press** and **roll**

##### adb shell input text "Some\ Text!"

*What it does*: Writes the text in the focused text input like it was being written in the keyboard, character by character.

*Cons*: You have to escape the spaces and if you need to paste some weird characters like emoji it might not be supported.

##### adb shell input tap 394 2098

*What it does*: Performs a touchscreen tap in a specific screen coordinate.

*Cons*: You have to fumble around into getting the correct coordinates to do what you want.

##### adb shell input swipe 540 1600 540 1800 200

*What it does*: Performs a touchscreen swipe from a screen coordinate into another with a set duration.

*Cons*: Has the same as **input** but now you also have to consider the swipe duration (last argument).

##### adb shell input keyevent 66

*What it does*: Android devices have special keys, for example the home button or the volume up and down buttons. This command triggers an event for that key as if they pushed by the user.

*Cons*: You will have to search for the integer that correspond to they key. You can solve this by creating alias for the values in this file: https://cs.android.com/android/platform/superproject/+/master:bionic/libc/kernel/uapi/linux/input-event-codes.h

##### adb shell input keyevent press and roll
You can look into the remaining commands bu typing:

`adb shell input`

At this point the remaining are **press** and **roll** which performs actions over the trackball that Android devices used to have.

#### Making adb shell input a little bit better


TODO: 

- [ ] Get-view-centers
- [ ] Replicating events - adb shell getevent/sendevent ; custom binary
- [ ] Accessibility service.
- [ ] uiautomator and starting it from the command line
- [ ] Also mention python monkey
