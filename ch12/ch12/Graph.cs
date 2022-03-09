namespace ch12;

public class Node
{
    public List<Node> Connections = new();
    public string Name = String.Empty;

    public bool IsSmall()
    {
        return Name.Length > 0 && Char.IsLower(Name[0]);
    }
}

public class Graph
{
    public Node Start;
    public Node End;

    public Graph(string[] connections)
    {
        var nodes = new Dictionary<string, Node>();
        foreach (var connection in connections)
        {
            var parts = connection.Split('-', 2);
            var from = parts[0];
            var to = parts[1];
            
            if (!nodes.ContainsKey(from))
            {
                var node = new Node()
                {
                    Connections = new List<Node>(),
                    Name = from
                };

                nodes.Add(from, node);
            }
            
            if (!nodes.ContainsKey(to))
            {
                var node = new Node()
                {
                    Connections = new List<Node>(),
                    Name = to
                };

                nodes.Add(to, node);
            }

            var fromNode = nodes[from];
            var toNode = nodes[to];

            //  Make sure start/end nodes are unidirectional
            if (from == "start" || to == "end")
            {
                fromNode.Connections.Add(toNode);
            } else if (to == "start" || from == "end")
            { 
                toNode.Connections.Add(fromNode);
            }
            else
            {
                fromNode.Connections.Add(toNode);    
                toNode.Connections.Add(fromNode);    
            }
        }

        Start = nodes["start"];
        End = nodes["end"];
        _allPaths = new();
    }
    
    private List<List<Node>> _allPaths;

    private void FindToEnd(Node startNode, List<Node> visitedSmall, List<Node> currentPath)
    { 
        currentPath.Add(startNode);

        foreach (var neighbor in startNode.Connections)
        {
            if (neighbor.IsSmall() && visitedSmall.Contains(neighbor))
            {
                continue;
            }

            if (neighbor == End)
            {
                var finalPath = new List<Node>(currentPath) { neighbor };
                _allPaths.Add(finalPath);
                continue;
            }

            var path = new List<Node>(currentPath);
            var visited = new List<Node>(visitedSmall);
            
            if (neighbor.IsSmall())
            {
                visited.Add(neighbor);
            }
            
            FindToEnd(neighbor, visited, path);
        }
    }
    
    private void FindToEndPart2(Node startNode, List<Node> visitedSmall, List<Node> currentPath, bool visitedSmallTwice)
    { 
        currentPath.Add(startNode);

        foreach (var neighbor in startNode.Connections)
        {
            if (neighbor.IsSmall() && visitedSmall.Contains(neighbor) && visitedSmallTwice)
            {
                continue;
            }

            if (neighbor == End)
            {
                var finalPath = new List<Node>(currentPath) { neighbor };
                _allPaths.Add(finalPath);
                continue;
            }

            var path = new List<Node>(currentPath);
            var visited = new List<Node>(visitedSmall);
            var flag = visitedSmallTwice;
            
            if (neighbor.IsSmall())
            {
                if (visited.Contains(neighbor) && !flag)
                {
                    flag = true;
                } else if (!visited.Contains(neighbor))
                {
                    visited.Add(neighbor);
                }
            } 
            
            FindToEndPart2(neighbor, visited, path, flag);
        }
    }
    
    private bool VerifyPath(List<Node> path)
    {
        if (path.First() != Start)
        {
            Console.WriteLine("Path does not start at 'Start' node!");
        }
        if (path.Last() != End)
        {
            Console.WriteLine("Path does not end at 'End' node!");
        }
        
        var smallVisited = new List<Node>();
        var temp = new List<Node>(path);
        
        //  Remove start node
        temp.RemoveAt(0);

        var result = true;
        Node current = Start;
        while (temp.Count > 0)
        {
            if (current == End)
            {
                Console.WriteLine($"Path contains stray nodes! (left: '{temp.Count}')");
                result = false;
            }
            
            Node next = temp.First();
            temp.RemoveAt(0);

            if (next.IsSmall() && smallVisited.Contains(next))
            {
                Console.WriteLine($"Path visits a small cave ('{next.Name}') multiple times!");
                result = false;
            }
            
            if (next.IsSmall() && !smallVisited.Contains(next))
            {
                smallVisited.Add(next);
            }

            if (!current.Connections.Contains(next))
            {
                Console.WriteLine($"Node '{current.Name}' does not contain a neighbor called '{next.Name}'!");
                result = false;
            }
            
            current = next;
        }

        if (current != End)
        {
            Console.WriteLine($"Path did not lead to End node! Got: {current.Name}");
            result = false;
        }

        return result;
    }
    
    private void VerifyResults()
    {
        foreach (var path in _allPaths)
        {
            var ret = VerifyPath(path);
            if (!ret)
            {
                Console.Write("Path '");
                foreach (var node in path)
                {
                    Console.Write($"{node.Name},");
                }
                Console.WriteLine("' invalid!");
            }
            
        }
    }
    
    public void FindAllPaths()
    {
        // FindToEnd(Start, new List<Node>(), new List<Node>());
        FindToEndPart2(Start, new List<Node>(), new List<Node>(), false);

        foreach (var path in _allPaths)
        {
            foreach (var node in path)
            {
                 Console.Write($"{node.Name},");
            }
            
            Console.WriteLine(); 
        }
        
        // Console.WriteLine("Verifying all paths");
        // VerifyResults();
        
        Console.WriteLine($"Total paths: {_allPaths.Count}");

        var count = 0;
        foreach (var path in _allPaths)
        {
            var n = path.Count(node => node.IsSmall() && node.Name is not ("start" or "end"));
            if (n <= 1)
            {
                count += 1;
            }
        }
        Console.WriteLine($"Paths with <= 1 small cave visits: {count}");
    }
    
    
}