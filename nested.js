var myPlants = [
    {
        type: "flowers",
        list: [
            "rose",
            "tulip",
            "dandelion"
        ]
    },
    {
        type: "trees",
        list: [
            "fir",
            "pine",
            "birch"
        ]
    }
];

var secondTree = myPlants[1].list[1]; // Change this line

// Only change code above this line
console.log(secondTree); // pine

//explaination:
// We can access the sub-properties of objects by chaining together the dot or bracket notation.
// This returns the value of the second element in the list key inside the second object in the myPlants array.
// The value is the string "pine".
