import QtQuick 2.6
import QtQuick.Window 2.0

Window {
    visible: true

    Text {
        anchors.centerIn: parent
        // Call a method
        text: greeter.compute_greetings("hello")
    }
}