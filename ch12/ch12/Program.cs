using ch12;

var lines = File.ReadAllLines("input.txt");
var graph = new Graph(lines);
graph.FindAllPaths();
