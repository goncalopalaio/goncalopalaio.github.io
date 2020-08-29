title = Scripting with automation in Android - Part 1
date = 15-08-2020

What do you do when you want to automate something during development but you do not want to leave behind changes in the project? Or you don’t even have instrumented tests configured?

I think this is an unexplored topic since it’s pretty non-standard. I will ignore the fact that you could add uiautomator or espresso to your project and perform your actions there. What I am interested is in solutions where it is not required that you make changes to your project.

I will try to make this as brief as I can. I won't delve too much on what each command argument means.

Let's build arbitrary examples using an application from Play Store.

I will use [Notally | Minimalist Notes](https://play.google.com/store/apps/details?id=com.omgodse.notally), a fine note taking application.


# 1 - Using `adb shell input`

Let's say I wanted to create a new note after opening the application.

<figure class="video_container">
  <video controls="true" allowfullscreen="true" poster="vid/adb_input_1.jpg">
	<source src="vid/adb_input_1.mp4" type="video/mp4">
	<source src="vid/adb_input_1.webm" type="video/webm">
		<source src="vid/adb_input_1.ogg" type="video/ogg">
  </video>
</figure>

#### Launching an application

First, you need to launch the application:

	PACKAGE="com.omgodse.notally"

	# One of the ways to launch an application with its package name
	adb shell monkey -p $PACKAGE -c android.intent.category.LAUNCHER 1
	
	# Alternative:
	# adb shell pm dump $PACKAGE | grep -E "intent=.*$PACKAGE"
	#
	# > intent={act=android.intent.action.MAIN cat=[android.intent.category.LAUNCHER] flg=0x10200000 cmp=com.omgodse.notally/.activities.MainActivity}
	#
	# adb shell am start -n "com.omgodse.notally/.activities.MainActivity" -a android.intent.action.MAIN -c android.intent.category.LAUNCHER

#### Tapping views

To create a note you have to tap the floating action button at the bottom. To do that you need its screen coordinates.

Either dump the view layout:

	adb shell uiautomator dump --compressed && adb pull /sdcard/window_dump.xml views.xml && xmllint --format views.xml
	
	 > <node NAF="true" index="1" text="" resource-id="com.omgodse.notally:id/TakeNoteFAB" class="android.widget.ImageButton" package="com.omgodse.notally" content-desc="" checkable="false" checked="false" clickable="true" enabled="true" focusable="true" focused="false" scrollable="false" long-clickable="false" password="false" selected="false" bounds="[1188,2812][1384,3008]"/>

The view bounds are 1188,2812 to 1384,3008. You will need to tap somewhere around the center of the view bounds:

	adb -s b8435fb0 shell input tap 1286 2910

I've created a small script to make this process a little bit easier: *adb-get-view-center.py* [here](https://github.com/goncalopalaio/basher-py).

	# You only need to provide the view id or the text the view has. This script will the dump the views give you the view center.

	python3 adb-get-view-center.py -i TakeNoteFAB
	> adb shell input tap 1286 2910
	
	python3 adb-get-view-center.py -i "Take note"
	> adb -s b8435fb0 shell input tap 720 2966
	
#### Writing text

Writing text is straightforward:

	adb shell input text "$RANDOM\ This\ Is\ Escaped\ Text!!"

You will need to escape the text and you're limited to ASCII characters, you're out of luck if you need to write complex characters. 

#### Key events

If you want to use the special keys that every device has you use:

	KEYCODE_BACK=4
	adb shell input keyevent $KEYCODE_BACK

You'll see that you need to know the event codes that each key corresponds. You can see the list of valid key codes here: [KeyEvent.java](https://cs.android.com/android/platform/superproject/+/master:frameworks/base/core/java/android/view/KeyEvent.java)

#### Other commands

There's other commands that you can list by running:
	
	adb shell input

For example you can simulate screen swipes by providing the duration, start and end coordinates of the swipe. I never found *swipe* to be useful, you're better off getting an alternative if you need to swipe or scroll a particular view reliably (see below).
 

# Making adb shell input a little bit better

If you really need string several of those commands together you will find that you most likely need to find where particular views are on the screen.
My biggest issue with adb shell input is that I always have to out of my way to find the screen coordinates by enabling the setting in the developer options that shows the coordinates on the screen.

I normally use: 

	adb shell uiautomator dump --compressed && adb pull /sdcard/window_dump.xml views.xml && xmllint --format views.xml

Which it will print the view tree as xml and will allow you to look for the screen coordinates of a particular view that you want, but you still have to search around the xml for what you want.

There might be some issues when dealing with overlapping activities if you simply run this command. It will work for most cases but if it doesn't you can also use [Layout Inspector](https://developer.android.com/studio/debug/layout-inspector) in Android Studio.

Another alternative to check the screen coordinates is:

	adb shell getevent -l

You will get a log of the events that the device sensors are receiving. You will notice that the values are in hexadecimal so you'll have to do that conversion first.

To make things a little better for myself I've created two scripts for both cases:

	python3 adb-get-view-center.py -i "EnterTitle"
		# Device: b8435fb0
		adb -s b8435fb0 shell input tap 720 398

	adb shell getevent -l | python3 adb-getevent.py
		X -> 950
		Y -> 1769
		X -> 939
		Y -> 1754

Source in: [basher-py](https://github.com/goncalopalaio/basher-py)

- *adb-get-view-center* will give you the screen coordinates of the view id or view text that you provide as an argument to the script.
- *adb-getevent* will print the screencordinates as an integer. There's an alternative mode, but more on that in the following section.

You can also get additional events with: 

	adb shell uiautomator events

Which lists some of the accessibility events occurring at the moment such as if any view content changed in the screen or anything was scrolled in the screen.

## Replicating events through getevent and sendevent

There's a lot of scattered information on this throughout the internet.

There are three limitations by default that prevent you simply saving what getevent gives you and piping it directly to sendevent.
First of all, getevent will not provide you timing event so you know the user touched the screen, but not at what point after you started recording the events. Second, getevent uses a different format than sendevent so you will have to convert the events first.

You can look at the hex representation of getevent events at [input-event-codes.h](https://cs.android.com/android/platform/superproject/+/master:bionic/libc/kernel/uapi/linux/input-event-codes.h)

The third limitation is that sendevent is slow to send events as it looks like it was never meant to be used to send multiple events. It opens the file descriptor that represents the device sensor, sends a single event and then closes the file descriptor (see [stackoverflow/adb-drag-vs-swipe-manual-drag-via-events](https://stackoverflow.com/questions/54505498/adb-drag-vs-swipe-manual-drag-via-events/54547196#54547196)).

To get around this, a few smart folks had the idea of recompiling sendevent so it would receive several events and write in one shot to the device sensor file descriptor.

So here's the thing, you cannot have executables in the internal storage of the device (assuming that your device is not rooted). It makes sense you wouldn't be able to. But how to get around this? Apparently you're allowed to push to */data/local/tmp/* and then call the executable with adb (not from the device itself).

	adb push myprogram /data/local/tmp/ && adb shell /data/local/tmp/myprogram

These are the two main examples where I saw this being done:

https://github.com/Cartucho/android-touch-record-replay/

https://github.com/rils/ARP/wiki

I've also found this by accident in an android internal tool: [android/autotest/client/bin/input/](https://cs.android.com/android/platform/superproject/+/master:external/autotest/client/bin/input/) but surely it won't be as easy to make it work.

# Monkeyrunner

You also have [monkeyrunner](https://developer.android.com/studio/test/monkeyrunner) which is a python API to control your devices. Not to be confused with [Monkey](https://developer.android.com/studio/test/monkey), another program that generates random user events in the device to perform stress testing or *monkey testing* as some people call it.

You can simulate taps, drags, write text, launch activities, take screenshots and compare screenshots. It seems perfectly fine, I don't have much experience with it.

# Automation suites in the device

Even if you do not include instrumented tests in your application project there are other ways to get access to your device.

## Creating a accessibility service

If you're annoyed that you always have to write the exactly same text in the exact same place, you could create an accessibility service just for this. I know it's kind of abusing the original intent of the tool, but it works.
While active, it will continuously run and look and do what you want with the views it is seeing.
Keep in mind that this will go through all of your views attributes and might even reveal some problems dealing with accessibility services. That might even be a bonus.

==TODO link to example project==


## Using UiAutomator

This is the most flexible of the previous options. You pretty much have control of all of your device. The only downside over the accessibility service is that you have to start the actions explicitly. 

This is normally used in a project so you can do interactions outside of your application but you can also take advantage of this and make it a generic way of performing any action at any time.

For this you have to create a separate project and run your tests which will install a special test apk. This creates a special entry point for the test that you wrote and allows you to start a specific test:

	adb shell am instrument -w -r -e debug false -e class 'com.example.ExampleTest' com.example.test/androidx.test.runner.AndroidJUnitRunner


### Make the tests dynamic

Instead of having a particular set of actions encoded in your tests you can pass custom parameters to the test and have a single test perform different actions.

Here’s an example project that I created:

[goncalopalaio/DynamicTester](https://github.com/goncalopalaio/DynamicTester)

You could even embed a small language interpreter, for example LUA, add some bindings to the uiautomator methods, send a program as a parameter to the test and have it run within the test. The extreme extension of that idea is:

[xiaocong/uiautomator](https://github.com/xiaocong/uiautomator)

Which as far as I know starts a test and blocks it forever to receive commands. You then have server running in the device that will do what an external python program tells it to in a RPC kind of way.

The downside that I’ve noticed when I used it is that it has to do communication both ways and it can be slow to do it.