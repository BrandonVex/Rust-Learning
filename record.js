var collection = { // this is the property of the collection object
    "2548": { // this is the id of the collection object
      "album": "Slippery When Wet",
      "artist": "Bon Jovi",
      "tracks": [ 
        "Let It Rock", 
        "You Give Love a Bad Name" 
      ]
    },
    "2468": {
      "album": "1999",
      "artist": "Prince",
      "tracks": [ 
        "1999", 
        "Little Red Corvette" 
      ]
    },
    "1245": {
      "artist": "Robert Palmer",
      "tracks": [ ]
    },
    "5439": {
      "album": "ABBA Gold"
    }
};
// Only change code below this line

var collectionCopy = JSON.parse(JSON.stringify(collection));   

function updateRecords(id, prop, value) {
    if (value === "") {
        // If the value is empty, we delete the prop property from the album
        delete collection[id][prop];
    // if property is tracks we are going to push on the end of the album's existing tracks array
    } else if (prop === "tracks") {
        // If the prop is "tracks" but the album doesn't have a "tracks" property, we create an empty array before adding the new value to the album's corresponding property
        collection[id][prop] = collection[id][prop] || [];
        // If the prop is "tracks" and the value isn't empty, we push the value onto the end of the album's existing tracks array
        collection[id][prop].push(value);
    // If the prop isn't "tracks" and the value isn't empty, we update or set the value for that record album's property
    } else {
        // If the prop isn't "tracks" and the value isn't empty, we update or set the value for that record album's property
        collection[id][prop] = value;
    }
    return collection;
}

//steps:
// 1. If prop isn't "tracks" and value isn't empty (""), update or set the value for that record album's property.
// 2. If prop is "tracks" but the album doesn't have a "tracks" property, create an empty array before adding the new value to the album's corresponding property.
// 3. If prop is "tracks" and value isn't empty (""), push the value onto the end of the album's existing tracks array.
// 4. If value is empty (""), delete the given prop property from the album.

updateRecords(5439, "artist", "ABBA");

