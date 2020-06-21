title = Face keypoint detection
date = 22-04-2018

[Imported from an old blog. 25/08/2019]

This hobby project started as a solution to the now finished [Facial Keypoints detection kaggle competition](https://www.kaggle.com/c/facial-keypoints-detection/)

While messing around with the dataset and reading the competition goal I realized that I was not really interested in the outcome. The original competition only has you predicting a few keypoints and not all images are annotated equally. The competition is over so that's no fun also.

In the meantime I wrote some code.
I've written a small portion of code to create a model with Tensorflow, save it and restore it and visualize some results in Tensorboard. It's pretty much untested and I will likely rewrite it in future dives.
I've implemented a linear regression model with sklearn and make it work in an android application. There's no surprise in saying that it works pretty badly. 

I just wanted to see it working. Of course using only linear regression, makes it really easy to reimplement in android application, since you only have to do a matrix multiply and add the bias.

I've got some convolution code running on android application somewhere, so the next step will be creating a convolutional neural network to do the same work, but this time with a custom loss function, so it jointly optimizes all keypoints. The linear regression model that I implemented has 30 individual models. I don't know how much the model has to gain by jointly optimizing but I really want to try it that way. 

Also implemented a way to visualize a bunch of images and their respective keypoints. I wanted something custom instead of using JupyterLabs/IPython. Since I've been trying to learn rust. I made a small program in rust, started with glium to render a few textures but later moved to a software renderer using mini-fb, since it is really simple to handle and don't have to handle the opengl mess.

The project is now in draft stage.
I started looking into alternative datasets to use.

I'll probably use [StephenMilborrow/muct](https://github.com/StephenMilborrow/muct)

[face-keypoint-detection](https://github.com/goncalopalaio/face-keypoint-detection)

-- April 22 2018