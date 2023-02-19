Link to the Nifty Project: http://nifty.stanford.edu/2018/ventura-hurricane-tracker/nifty-hurricanes.html

Will be a non-standard implementation of the project. The rust turtle library does not have all of the features afforded by the python turtle library (the target of the example). As such:
    - The turtle will not be represented by a hurricane
    - There will be no background to the turtle animation
There are a few other consequences here (ex. Don't have the world coordinate function), but a best effort is made to fit with the spirit of the project without having to write my own turtle implementation.

However, because the turtle drawing can be saved as an svg, that will be used to overlay the hurricane path onto the provided image.