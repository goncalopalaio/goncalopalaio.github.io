title = Scripting with automation in Android
date = 15-08-2020

Let's start by saying that you should always strive to have  unit and instrumented tests in your projects. But for cases where you see yourself doing the same actions over and over again while developing, there are ways to automate those actions.

Some of the sections assume that you have some experience with android development.


# 1 - Using `adb shell`

There are a multitude of commands that `adb shell` that enables actions over the connected device.

I've created this [Example](https://gist.github.com/goncalopalaio/1326ad423353485d64f71139b33235fd) that launches and creates a new note in [Notally | Minimalist Notes](https://play.google.com/store/apps/details?id=com.omgodse.notally).

<figure class="video_container">
  <video controls="true" allowfullscreen="true" poster="vid/adb_input_1.jpg">
	<source src="vid/adb_input_1.mp4" type="video/mp4">
	<source src="vid/adb_input_1.webm" type="video/webm">
		<source src="vid/adb_input_1.ogg" type="video/ogg">
  </video>
</figure>

The downside of this approach is that you rely on the view coordinates, for that reason, the script must be adapted to each device. Using `adb shell uiautomator dump` is one of the easiest ways to check at which coordinates a view is (the other is the [Android Studio Layout Inspector](https://developer.android.com/studio/debug/layout-inspector)).

To further facilitate taking view coordinates, I created [adb-get-view-center.py](https://github.com/goncalopalaio/basher-py) which filters the `adb shell uiautomator dump` command and gives you the center view coordinates of a particular view.

You'll notice that for key events, you will need to provide the key code of the key. You can see the list of valid key codes here: [KeyEvent.java](https://cs.android.com/android/platform/superproject/+/master:frameworks/base/core/java/android/view/KeyEvent.java)

There's at least two more ways to get view coordinates. You will find a way to show the user touch coordinates in the device developer options and you can deduce the view coordinates from them.

Alternatively you can use 	`adb shell getevent -l` which will output sensor data.
You will get a log of the events that the device sensors are receiving. You will notice that the values are in hexadecimal so you'll have to do that conversion first.

In [adb-getevent.py](https://github.com/goncalopalaio/basher-py/blob/master/adb-getevent.py) you'll find a way to convert the values into screen coordinates.

	adb shell getevent -l | python3 adb-getevent.py
		X -> 950
		Y -> 1769
		X -> 939
		Y -> 1754


Note that the output of `adb shell getevent -l` looks like it's a direct output of what the sensors are receiving but in reality the output is buffered. It might take a while to see the latest touch events. 

You can get additional events with: 

	adb shell uiautomator events

Which will output a limited set of the accessibility events occurring at the moment such as if any view content changed or if any view was scrolled.


## Replicating events through  `getevent`  and `sendevent`

In theory, you could take what `getevent` gives you and feed it into `sendevent` but there are caveats.

First, `getevent` will not provide you timing events to determine when the event happened (and you'll have to deal with output buffering).

Second, `getevent` uses a different format than `sendevent`, you will have to convert the events first.

You can look at the hex representation of `getevent` events at [input-event-codes.h](https://cs.android.com/android/platform/superproject/+/master:bionic/libc/kernel/uapi/linux/input-event-codes.h)

The third limitation is that `sendevent` is slow to send events as it looks like it was never meant to be used to send multiple events. It opens the file descriptor that represents the device sensor, sends a single event and then closes the file descriptor (see [stackoverflow/adb-drag-vs-swipe-manual-drag-via-events](https://stackoverflow.com/questions/54505498/adb-drag-vs-swipe-manual-drag-via-events/54547196#54547196)).

To get around this, a few smart folks had the idea of recompiling `sendevent` to receive several events and write in one shot to the device sensor file descriptor.

You cannot have executables in the internal storage of the device (assuming that your device is not rooted). It makes sense you wouldn't be able to. But how to get around this? Turns out you're allowed to push to */data/local/tmp/* and then call the executable with adb (not from the device itself).

	adb push myprogram /data/local/tmp/ && adb shell /data/local/tmp/myprogram

These are examples where I saw this being done:

- https://github.com/Cartucho/android-touch-record-replay/

- https://github.com/rils/ARP/wiki
 
- [android/autotest/client/bin/input/](https://cs.android.com/android/platform/superproject/+/master:external/autotest/client/bin/input/)

# Monkeyrunner

[Monkeyrunner](https://developer.android.com/studio/test/monkeyrunner) which is a python API to control your devices. Not to be confused with [Monkey](https://developer.android.com/studio/test/monkey), which is another separate program that generates random user events in the device to perform stress testing or *monkey testing* as people call it.

With Monkeyrunner uou can simulate taps, drags, write text, launch activities, take screenshots and compare screenshots. It's perfectly fine but I don't have much experience with it.

# Automation suites in the device

Even if you do not include instrumented tests in your application project there are other ways to get access to your device.

## Creating a accessibility service application

If you're annoyed that you always have to write the same text in the exact same place, you could create an accessibility service just for this. It's abusing the original intent of the tool, I know, but it works.
While active, it will continuously run and look and do what you want with the views it's seeing.
Keep in mind that this will go through all of your views attributes and might even reveal problems dealing with accessibility services. I've had crashes in RecyclerView's while using this that revealed accessibility issues.

## Using a separate project with UIAutomator tests

This is the most flexible of the previous options. You can have control of almost everything in your device. The downside over a custom accessibility service is that you have to start the actions explicitly. 

This is normally used in a project to do interactions outside of your project application but you can take advantage of this and make it a generic way of performing any action at any time and place.

You'll have to create a separate project, create the tests with your actions and run the tests. Running the tests the first time will install a test APK. This creates a special entry point to run the test you wrote at any time you want, provided that you have the test APK installed, for example:

	adb shell am instrument -w -r -e debug false -e class 'com.example.ExampleTest' com.example.test/androidx.test.runner.AndroidJUnitRunner


### Make the tests dynamic

Instead of having a particular set of actions encoded in your tests, there's the possibility to pass custom parameters to the test and have a single test perform different actions.

You can check this example I created:

[dynamictester/DynTest.kt](https://github.com/goncalopalaio/DynamicTester/blob/master/app/src/androidTest/java/com/gplio/dynamictester/DynTest.kt)

You could even embed a small language interpreter, for example LUA, add some bindings to the uiautomator methods, send a program as a parameter to the test and have it run within the test. The extreme extension of that idea is:

[xiaocong/uiautomator](https://github.com/xiaocong/uiautomator)

Which appears to start a test and block it forever to receive commands. You then have server running in the device that will do what an external python program tells it to in a RPC kind of way.

The downside that I’ve noticed when I used it is that it has to do communication both ways and it can be slow to do it.

### Using accessibility events in the tests

Creating an accessibility service application has a lot of setup. Wouldn't it be nice to do the same thing in an instrumented test? You can, we will cheat a little bit by not letting the test finish.

In [dynamictester/AccessibilityLoggerTest.kt](https://github.com/goncalopalaio/DynamicTester/blob/master/app/src/androidTest/java/com/gplio/dynamictester/AccessibilityLoggerTest.kt) you'll see an instrumented test that starts listening to accessbility events continuously

While the following command is running, when an EditText view is focused, it will write "Hello" in it without further user intervention:

	adb shell am instrument -w -r    -e debug false -e class 'com.gplio.dynamictester.AccessibilityLoggerTest' com.gplio.dynamictester.test/androidx.test.runner.AndroidJUnitRunner
