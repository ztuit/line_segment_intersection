 use uuid::Uuid;


fn main() {
    //let mypoint1 = MyPoint::new(1,1);
    //let mypoint2 = MyPoint::new(2,2);
    //let myLine = MyLine:: new(&mypoint1,&mypoint2);
    //let mypoint3 = &mypoint1;
    //myLine.draw();
    //mypoint1.draw();

    //Define a variable, it is now owned by variable in the decleration
    let original_owner:String = String::from("abc");
    print!("\nOriginal Owner of {}\n", original_owner);
    //Give ownership to a new owner
    let mut new_owner:String = original_owner;
    print!("\nNew owner of {}\n",new_owner);
    //Let a variable borrow the variable
    {
        let borrower:&String = &new_owner;
        print!("\n Borrower of {}\n", borrower);
        let second_borrower:&String = &new_owner;
        print!("\n Second Borrower of {}\n", second_borrower);
    }
    print!("\nNew Owner of {}\n", new_owner);
    //Let the variable be borrowed to change
    let change_borrower:&mut String = &mut new_owner;
    change_borrower.push_str("def");
    print!("\n Change Borrower of {}\n", change_borrower);
     print!("\nNew owner of {}\n",new_owner);
    
    //print!("\nsecond borrow {}\n", secondborrow);
    //let mut returnVal:u32 = test_reference(&mut input);
    //print!("\nnow input is {}\n", input)
    //print!("\nOrigainl value is {}", returnVal);
}

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Default)]
struct TreeNode {
    id: String,
    left:  Option<Box<Self>>,
    right: Option<Box<Self>>,
    event: ScanEvent
}

impl TreeNode {
    fn new(iid:String, l:Option<Self>,r:Option<Self>, e:ScanEvent) -> TreeNode {

        let lft : Option<Box<TreeNode>> = match l {
            Some(value) => Some(Box::new(value)),
            None => None
        };

        let rght : Option<Box<TreeNode>> = match r {
            Some(value) => Some(Box::new(value)),
            None => None
        };

        TreeNode{id:iid,right:rght, left:lft, event:e}
    }

    fn new_id(l:Option<Self>, r:Option<Self>, e:ScanEvent) -> TreeNode {
        let id = Uuid::new_v4();
        TreeNode::new(id.to_string(),l,r,e)
    }

    /* fn add_event(&mut self, se:&'a ScanEvent) {
        //Compare verticies on this and the input and choose right or left
          
        //If left or right doesn't exist then create new treenode and set
        //If the node exists then recurse to see if this new event should be right or left
        //what do we do about intersections?
    }*/

     fn find_parent(&mut self, val:Vertex) -> &mut TreeNode {

           let cmp_vertex = self.event.vertx_for_event_type();

         if cmp_vertex.x<val.x {

            if self.right.is_none() == false{
                return self.right.as_mut().expect("Not none").find_parent(val)
            }
            
        } else {
             if self.left.is_none() == false {
                return self.left.as_mut().expect("Not none").find_parent(val)
            }
        }
        self
    }

    fn add_child(&mut self, child:TreeNode) {
        let own_vertex = self.event.vertx_for_event_type();
        let child_vertex = child.event.vertx_for_event_type();
        if own_vertex.x > child_vertex.x {
            self.left = Some(Box::new(child));
        } else {
            self.right = Some(Box::new(child));
        }
    }

}

#[derive(Debug)]
struct BinaryTree {
    root:TreeNode
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
enum EventType {
    #[default] START,
    END,
    INTERSECTION
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Default)]
struct ScanEvent{
    event_type:EventType,
    lines: Vec<Line>,
    intersection_point: Option<Vertex>
}

impl ScanEvent {

     fn new_event(event_type: EventType, line: Line) -> ScanEvent {
        ScanEvent { event_type, lines: vec![line], intersection_point: None }
    }

     fn new_intersection_event(intersection: Vertex, line1: Line, line2: Line) -> ScanEvent {
        ScanEvent { event_type: EventType::INTERSECTION, lines: vec![line1, line2], intersection_point: Some(intersection) }
    }

    fn vertx_for_event_type(&self) -> Vertex {
       match self.event_type {
            EventType::START => self.lines.get(0).unwrap().p1.clone(),
            EventType::INTERSECTION => { match &self.intersection_point {
                Some(value) => value.clone(),
                None => panic!("Intersection event without intersection point")
            }},
            EventType::END => self.lines.get(0).unwrap().p2.clone()
        }
    }
}

#[derive(Debug)]
struct ScanEvents {
    events:Vec<ScanEvent>
}

impl ScanEvents {

    fn new() -> ScanEvents {
        ScanEvents{events:Vec::new()}
    }

   

    fn add_event(mut self, se: ScanEvent) -> Self {
        //ensures the events points ordered left to right top to bottom
        let se_vertex = se.vertx_for_event_type();
        match self.events.iter().position(|probe| { 
            let probe_vertex = probe.vertx_for_event_type();
            probe_vertex.y>se_vertex.y || (probe_vertex.y==se_vertex.y && probe_vertex.x>se_vertex.x) }) {
            Some(value) => {
                self.events.insert(value, se);
            },
            None => { self.events.push(se); } //Empty vector, or add at end
        }
        
       
        self
    }

    
}

#[derive(Debug,Clone)]
#[derive(PartialEq)]
struct Vertex {
    x: f32,
    y: f32
}

#[derive(Debug,Clone)]
#[derive(PartialEq)]
struct Line {
    p1: Vertex,
    p2: Vertex,
    m: f32,
    intercept: f32
}

impl Line {

    fn new(p1:Vertex, p2: Vertex) -> Line {
        let xd = (p2.x-p1.x) as f32;
        let mut m : f32 = (p2.y-p1.y) as f32;
        if xd != 0.0 {
            m = (m/xd) as f32;
        }
        let xf : f32 = p1.x as f32;
        let yf : f32 = p1.y as f32;
        let intercept : f32 = yf - m*xf;
        Line{p1,p2,m,intercept}
    }

    fn length(self) -> f32 {
        let sum : f32 = ((self.p2.x-self.p1.x)+(self.p2.y-self.p1.y)) as f32;
        sum.sqrt()
    }

    fn intersects(&self,l2:Line) -> Option<Vertex> {
        let x1 = (l2.intercept-self.intercept)/(self.m - l2.m);
        let y1 = self.m*x1+self.intercept;
        let y2 = l2.m*x1+l2.intercept;
        //Check for the same value of x the y is the same
        if y1==y2 {
            Some(Vertex{x:x1,y:y2})
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_tree_node_with_empty() {
              let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1);
        let t = TreeNode::new("1".to_string(),None,None, scan_event);
        assert_eq!(t.right, None);
        assert_eq!(t.left, None);
    }

      #[test]
    fn test_create_tree_node_with_right() {
              let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_event2 = ScanEvent::new_event(EventType::START, l1.clone());
        let t = TreeNode::new("0".to_string(),None,None,scan_event);
        let s = Some(t);
        let t1 = TreeNode::new("1".to_string(),None,s,scan_event2);
        assert_eq!((*t1.right.expect("not none")).id, "0");
        assert_eq!(t1.left, None);
    }

    
      #[test]
    fn test_create_tree_node_with_left() {
              let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_event2 = ScanEvent::new_event(EventType::START, l1.clone());
        let t = TreeNode::new("1".to_string(),None,None, scan_event);
        let s = Some(t);
        let t1 = TreeNode::new("0".to_string(),s,None, scan_event2);
        assert_eq!((*t1.left.expect("not none")).id, "1");
        assert_eq!(t1.right, None);
    }

      #[test]
    fn test_create_tree_node_with_left_and_right() {
         let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_event2 = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_event3 = ScanEvent::new_event(EventType::START, l1.clone());
        let l = TreeNode::new("2".to_string(),None,None, scan_event);
        let r = TreeNode::new("1".to_string(),None,None,scan_event2);
        let sl = Some(l);
        let sr = Some(r);
        let t1 = TreeNode::new("0".to_string(),sl,sr,scan_event3);
        assert_eq!((*t1.left.expect("Not null")).id, "2");
        assert_eq!((*t1.right.expect("Not null")).id, "1");
    }

    #[test]
    fn test_id_tree_node_with_left_and_right() {
        let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_event2 = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_event3 = ScanEvent::new_event(EventType::START, l1.clone());
        let l = TreeNode::new("2".to_string(),None,None, scan_event);
        let r = TreeNode::new("1".to_string(),None,None, scan_event2);
        let sl = Some(l);
        let sr = Some(r);
        let t1 = TreeNode::new("0".to_string(),sl,sr, scan_event3);
        match t1.left {
            Some(value) => assert_eq!(value.id, "2"),
            None => assert_eq!(0,1),
        }
        match t1.right {
            Some(value) => assert_eq!(value.id, "1"),
            None => assert_eq!(0,1),
        }
    }

    #[test]
    fn test_replace_tree_node_left_with_new() {
        let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_event2 = ScanEvent::new_event(EventType::START, l1.clone());
         let scan_event3 = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_event4 = ScanEvent::new_event(EventType::START, l1.clone());
        let l = TreeNode::new("2".to_string(),None,None,scan_event);
        let r = TreeNode::new("1".to_string(),None,None,scan_event2);
        let sl = Some(l);
        let sr = Some(r);
        let mut t1 = TreeNode::new("0".to_string(),sl,sr, scan_event3);
        let ln = TreeNode::new("3".to_string(),None,None,scan_event4);
        t1.left = Some(Box::new(ln));
        match t1.left {
            Some(value) => assert_eq!(value.id, "3"),
            None => assert_eq!(0,1),
        }
    }

    #[test]
    fn test_increase_tree_depth() {
         let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event1 = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_event2 = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_event3 = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_event4 = ScanEvent::new_event(EventType::START, l1.clone());
        let l = TreeNode::new("2".to_string(),None,None, scan_event1);
        let r = TreeNode::new("1".to_string(),None,None, scan_event2);
        let sl = Some(l);
        let sr = Some(r);
        let mut t1 = TreeNode::new("0".to_string(),sl,sr, scan_event3);
       
        let ln = TreeNode::new("3".to_string(),Some(*(t1.left.take().expect("not none"))),None, scan_event4);
        t1.left = Some(Box::new(ln));
        match t1.left {
            Some(value) => {
                assert_eq!(value.id, "3");
                match value.left {
                    Some(subvalue) =>  assert_eq!(subvalue.id, "2"),
                    None => assert_eq!(0,1),
                }
            },
            None => assert_eq!(0,1),
        }
    }

   

    #[test]
     fn test_create_tree_with_root(){
        let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1);
        let root_node = TreeNode::new("0".to_string(),None,None, scan_event);
        let tree = BinaryTree{root:root_node};
        assert_eq!(tree.root.id,"0");
    }

    #[test]
    fn test_create_scan_event(){
        let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1);
        assert_eq!(scan_event.event_type, EventType::START);
        assert_ne!(scan_event.event_type, EventType::END);
    }

    #[test]
    fn test_create_tree_node_with_scan_event(){
         
                 let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1);
        let root_node = TreeNode::new("0".to_string(),None,None, scan_event);
        assert_eq!(root_node.event.event_type, EventType::START)
    }

    #[test]
    fn test_create_points(){
        let p1 = Vertex{x:1.0, y:1.0};
        assert_eq!(p1.x,1.0);
        assert_eq!(p1.y,1.0);
    }

    #[test]
    fn test_create_line(){
        let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        assert_eq!(l1.p1.x, 1.0);
        assert_eq!(l1.p2.y,2.0);
    }

    #[test]
    fn test_line_length(){
        let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:3.0,y:3.0};
        let l1 = Line::new(p_1, p_2);
        let length = l1.length();
        assert_eq!(length,2.0);
    }

    #[test]
    fn test_line_slope() {
        let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:3.0,y:3.0};
        let l1 = Line::new(p_1, p_2);
        assert_eq!(l1.m,1.0);
    }

    #[test]
    fn test_line_intercept() {
        let p_1 = Vertex{x:1.0, y:3.0};
        let p_2 = Vertex{x:3.0,y:5.0};
        let l1 = Line::new(p_1, p_2);
        assert_eq!(l1.intercept,2.0);
    }

    #[test]
    fn test_line_intersection(){
        let mut p_1 = Vertex{x:1.0, y:1.0};
        let mut p_2 = Vertex{x:3.0,y:3.0};
        let l1 = Line::new(p_1, p_2);
        p_1 = Vertex{x:1.0, y:3.0};
        p_2 = Vertex{x:3.0,y:1.0};
        let l2 = Line::new(p_1, p_2);
        let intersection = l1.intersects(l2);
        match intersection {
            Some(value) => {
                assert_eq!(value.x, 2.0);
                assert_eq!(value.y, 2.0);
            },
            None => assert_eq!(1,0)
        }
    }

    #[test]
    fn test_create_scan_events() {
        let _scan_events = ScanEvents::new();
    }

    #[test]
    fn test_add_scan_event(){
        let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1);
        let scan_events = ScanEvents::new();
        assert_eq!(scan_events.events.len(),0);
        let scan_events = scan_events.add_event(scan_event);
        assert_eq!(scan_events.events.len(),1);
    }

    #[test]
    fn test_add_multiple_scan_events(){
        let p_1 = Vertex{x:1.0, y:1.0};
        let p_2 = Vertex{x:2.0,y:2.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1);
        let p2_1 = Vertex{x:1.0, y:1.0};
        let p2_2 = Vertex{x:2.0,y:2.0};
        let l12 = Line::new(p2_1, p2_2);
        let scan_event2 = ScanEvent::new_event(EventType::START, l12);
        let scan_events = ScanEvents::new();
        assert_eq!(scan_events.events.len(),0);
        let scan_events = scan_events.add_event(scan_event);
        assert_eq!(scan_events.events.len(),1);
        let scan_events = scan_events.add_event(scan_event2);
        assert_eq!(scan_events.events.len(),2);
    }

    #[test]
    fn test_add_scan_event_inserted_correctly_y(){
        let p_1 = Vertex{x:1.0, y:10.0};
        let p_2 = Vertex{x:2.0,y:20.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1);
        let p2_1 = Vertex{x:1.0, y:5.0};
        let p2_2 = Vertex{x:2.0,y:2.0};
        let l12 = Line::new(p2_1, p2_2);
        let scan_event2 = ScanEvent::new_event(EventType::START, l12);
        let scan_events = ScanEvents::new();
        let scan_events = scan_events.add_event(scan_event);
        let scan_events = scan_events.add_event(scan_event2);
        match scan_events.events.get(0) {
            Some(value) => {
                assert_eq!(value.vertx_for_event_type().y, 5.0);
                
            },
            None => assert_eq!(1,0)
        }
        let p3_1 = Vertex{x:1.0, y:6.0};
        let p3_2 = Vertex{x:2.0,y:2.0};
        let l13 = Line::new(p3_1, p3_2);
        let scan_event3 = ScanEvent::new_event(EventType::START, l13);
        let scan_events = scan_events.add_event(scan_event3);
        match scan_events.events.get(1) {
            Some(value) => {
                assert_eq!(value.vertx_for_event_type().y, 6.0);
                
            },
            None => assert_eq!(1,0)
        }
        let p4_1 = Vertex{x:1.0, y:11.0};
        let p4_2 = Vertex{x:2.0,y:2.0};
        let l14 = Line::new(p4_1, p4_2);
        let scan_event4 = ScanEvent::new_event(EventType::START, l14);
        let scan_events = scan_events.add_event(scan_event4);
        match scan_events.events.get(3) {
            Some(value) => {
                assert_eq!(value.vertx_for_event_type().y, 11.0);
                
            },
            None => assert_eq!(1,0)
        }
    }

     #[test]
    fn test_add_scan_event_inserted_correctly_x(){
        let p_1 = Vertex{x:15.0, y:10.0};
        let p_2 = Vertex{x:2.0,y:20.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1);
        let p2_1 = Vertex{x:10.0, y:10.0};
        let p2_2 = Vertex{x:2.0,y:20.0};
        let l12 = Line::new(p2_1, p2_2);
        let scan_event2 = ScanEvent::new_event(EventType::START, l12);
        let scan_events = ScanEvents::new();
        let scan_events = scan_events.add_event(scan_event);
        let scan_events = scan_events.add_event(scan_event2);
        match scan_events.events.get(0) {
            Some(value) => {
                assert_eq!(value.vertx_for_event_type().x, 10.0);
                
            },
            None => assert_eq!(1,0)
        }
        let p3_1 = Vertex{x:6.0, y:10.0};
        let p3_2 = Vertex{x:2.0,y:2.0};
        let l13 = Line::new(p3_1, p3_2);
        let scan_event3 = ScanEvent::new_event(EventType::START, l13);
        let scan_events = scan_events.add_event(scan_event3);
        match scan_events.events.get(0) {
            Some(value) => {
                assert_eq!(value.vertx_for_event_type().x, 6.0);
                
            },
            None => assert_eq!(1,0)
        }
        let p4_1 = Vertex{x:7.0, y:10.0};
        let p4_2 = Vertex{x:2.0,y:2.0};
        let l14 = Line::new(p4_1, p4_2);
        let scan_event4 = ScanEvent::new_event(EventType::START, l14);
        let scan_events = scan_events.add_event(scan_event4);
        match scan_events.events.get(1) {
            Some(value) => {
                assert_eq!(value.vertx_for_event_type().x, 7.0);
                
            },
            None => assert_eq!(1,0)
        }
        let p4_1 = Vertex{x:4.0, y:11.0};
        let p4_2 = Vertex{x:2.0,y:2.0};
        let l14 = Line::new(p4_1, p4_2);
        let scan_event4 = ScanEvent::new_event(EventType::START, l14);
        let scan_events = scan_events.add_event(scan_event4);
        match scan_events.events.get(1) {
            Some(value) => {
                assert_eq!(value.vertx_for_event_type().x, 7.0);
                
            },
            None => assert_eq!(1,0)
        }
    }

    #[test]
    fn test_add_scan_event_considering_event_type_start_first(){
        let p_1 = Vertex{x:1.0, y:10.0};
        let p_2 = Vertex{x:2.0,y:20.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1);
        let p2_1 = Vertex{x:1.0, y:10.0};
        let p2_2 = Vertex{x:2.0,y:20.0};
        let l2 = Line::new(p2_1, p2_2);
        let scan_event2 = ScanEvent::new_event(EventType::END, l2);
        let scan_events = ScanEvents::new();
        let scan_events = scan_events.add_event(scan_event);
        let scan_events = scan_events.add_event(scan_event2);
        match scan_events.events.get(0) {
            Some(value) => {
                assert_eq!(value.event_type, EventType::START);
            },
            None => assert_eq!(1,0)
        }
    }

    #[test]
    fn test_add_scan_event_considering_event_type_end_first(){
        let p_1 = Vertex{x:1.0, y:10.0};
        let p_2 = Vertex{x:2.0,y:20.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::END, l1);
        let p2_1 = Vertex{x:1.0, y:10.0};
        let p2_2 = Vertex{x:2.0,y:20.0};
        let l2 = Line::new(p2_1, p2_2);
        let scan_event2 = ScanEvent::new_event(EventType::START, l2);
        let scan_events = ScanEvents::new();
        let scan_events = scan_events.add_event(scan_event);
        let scan_events = scan_events.add_event(scan_event2);
        match scan_events.events.get(0) {
            Some(value) => {
                assert_eq!(value.event_type, EventType::START);
            },
            None => assert_eq!(1,0)
        }
    }

    #[test]
    fn test_add_scan_event_considering_event_type_overlapping(){
        let p_1 = Vertex{x:1.0, y:10.0};
        let p_2 = Vertex{x:2.0,y:20.0};
        let l1 = Line::new(p_1, p_2);
        let scan_event = ScanEvent::new_event(EventType::END, l1.clone());
        let scan_event2 = ScanEvent::new_event(EventType::START, l1.clone());
        let p1_1 = Vertex{x:1.0, y:9.0};
        let p1_2 = Vertex{x:2.0,y:20.0};
        let l2 = Line::new(p1_1, p1_2);
        let scan_event3 = ScanEvent::new_event(EventType::END, l2.clone());
        let scan_event4 = ScanEvent::new_event(EventType::START, l2.clone());
        let scan_events = ScanEvents::new();
        let scan_events = scan_events.add_event(scan_event);
        let scan_events = scan_events.add_event(scan_event2);
        let scan_events = scan_events.add_event(scan_event3);
        let scan_events = scan_events.add_event(scan_event4);
        match scan_events.events.get(0) {
            Some(value) => {
                assert_eq!(value.event_type, EventType::START);
                assert_eq!(value.vertx_for_event_type().y,9.0);
            },
            None => assert_eq!(1,0)
        }
    }

    #[test]
    fn test_intersection_event_added_correctly(){
        let p1_1 = Vertex{x:10.0, y:1.0};
        let p1_2 = Vertex{x:1.0,y:10.0};
        let l1 = Line::new(p1_1, p1_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1.clone());
        let scan_events = ScanEvents::new();
        let scan_events = scan_events.add_event(scan_event);
        let scan_event = ScanEvent::new_event(EventType::END, l1.clone());
        let scan_events = scan_events.add_event(scan_event);
        let p1_1 = Vertex{x:1.0, y:1.0};
        let p1_2 = Vertex{x:10.0,y:10.0};
        let l2 = Line::new(p1_1, p1_2);
        let scan_event = ScanEvent::new_event(EventType::START, l2.clone());
        let scan_events = scan_events.add_event(scan_event);
        let scan_event = ScanEvent::new_event(EventType::END, l2.clone());
        let scan_events = scan_events.add_event(scan_event);
     
        let intersection = l1.intersects(l2.clone());
        let vertex = match intersection {
            Some(value) => {
                value
            },
            None => Vertex{x:0.0,y:0.0}
        };
        let intersection_event = ScanEvent::new_intersection_event(vertex, l1.clone(), l2.clone());
        let scan_events = scan_events.add_event(intersection_event);
        

        match scan_events.events.get(2) {
            Some(value) => {
                assert_eq!(value.event_type, EventType::INTERSECTION);
                match &value.intersection_point {
                    Some(v) => {
                        assert_eq!(v.x,5.5);
                        assert_eq!(v.y,5.5);
                    },
                    None => assert_eq!(1,0)
                }
            },
            None => assert_eq!(1,0)
        }
        
    }

  
    #[test]
    fn test_create_tree_from_scan_events() {
                let p1_1 = Vertex{x:10.0, y:1.0};
        let p1_2 = Vertex{x:1.0,y:10.0};
        let l1 = Line::new(p1_1, p1_2);
        let scan_event = ScanEvent::new_event(EventType::START, l1.clone());
       
        let scan_events = ScanEvents::new();
        let scan_events = scan_events.add_event(scan_event);
        let scan_event = ScanEvent::new_event(EventType::END, l1.clone());
        let scan_events = scan_events.add_event(scan_event);
        let p1_1 = Vertex{x:1.0, y:1.0};
        let p1_2 = Vertex{x:10.0,y:10.0};
        let l2 = Line::new(p1_1, p1_2);
        let scan_event = ScanEvent::new_event(EventType::START, l2.clone());
        let scan_events = scan_events.add_event(scan_event);
        let scan_event = ScanEvent::new_event(EventType::END, l2.clone());
        let mut scan_events = scan_events.add_event(scan_event);
        
        let mut more_elements = true;

        let mut  root_node =  match scan_events.events.pop() {
                Some(value) => {
                   Some(TreeNode::new("0".to_string(),None,None,value))
                },
                None => {
                    None
                }
            };
       
       

        while more_elements == true {
            match scan_events.events.pop() {
                Some(value) => {
                    //Create the root node if not created
                    let vertex = value.vertx_for_event_type().clone();
                    //Use the root node to find the parent
                    let parent_to_add_to : &mut TreeNode = root_node.as_mut().expect("not none").find_parent(vertex);
                    parent_to_add_to.add_child(TreeNode::new_id(None,None,value));
                },
                None => {
                    more_elements = false;}
                
            }
        }
    }

}
