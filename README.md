# oxiuse
It's a **graduation design** based on [oxigraph](https://github.com/oxigraph/oxigraph.git) which is a graph database implemented the SPARQL standard.

I will add the type information based on the encoding of the original triple, e.x. asubClassOfb,atypeb...It uses **interval coding** to determine whether there is a child-parent relationship between two classes.In order to obtain the interval encoding, the classes will be organized in the form of a multi-fork tree,and the way of interval encoding `[start,end]` is:
- `start`: the pre-order traversal number of node
- `end`: `start` plus the number of descendant nodes
 
 The oxigraph's original encoding schema of triple is:
 ![image](https://user-images.githubusercontent.com/52108493/164982506-6f840ab4-efc3-40dc-95d1-a4674ab1d034.png)
All my changes are for value,because the value there is empty,it's just like:
![image](https://user-images.githubusercontent.com/52108493/164982588-825e908b-e98a-448f-9259-70fa17217bce.png)
