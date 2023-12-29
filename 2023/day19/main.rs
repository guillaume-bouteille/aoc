

use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct WorkflowInstruction {
    variable : String,
    operand : String,
    value : i64,
    destination : String,
}

type Workflows = HashMap<String, Vec<WorkflowInstruction>>;

type Part = HashMap<String, i64>;


type Inputs = (Workflows,Vec<Part>);

fn parse_inputs() -> Inputs {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    let mut parsing_workflows : bool = true;
    for line_it in io::stdin().lines() {
        if let Ok(line) = line_it {
            if line == "" {
                parsing_workflows = false;
            } else {
                if parsing_workflows {
                    // Parse a workflow
                    let tu = line.split_once("{").expect("wouf");
                    let wf_name = tu.0;
                    let mut wf_instr_v = Vec::new();
                    for wf_instr in tu.1.to_string().replace("}", "").split(",") {
                        if let Some((comparison_stuff, destination)) = wf_instr.split_once(":") {
                            if let Some((variable, value)) = comparison_stuff.split_once("<") {
                                wf_instr_v.push(WorkflowInstruction{
                                    variable:variable.to_string(),
                                    operand:"<".to_string(),
                                    value: value.parse::<i64>().expect("truc"),
                                    destination:destination.to_string()
                                });
                            } else if let Some((variable, value)) = comparison_stuff.split_once(">") {
                                wf_instr_v.push(WorkflowInstruction{
                                    variable:variable.to_string(),
                                    operand:">".to_string(),
                                    value: value.parse::<i64>().expect("truc"),
                                    destination:destination.to_string()
                                });
                            } else {
                                todo!("{}", wf_instr);
                            }
                        } else {
                            wf_instr_v.push(WorkflowInstruction{
                                variable:String::new(),
                                operand:"*".to_string(),
                                value: 0,
                                destination:wf_instr.to_string()
                            });

                        }
                    }
                    workflows.insert(wf_name.to_string(), wf_instr_v);
                } else {
                    // Parse a part
                    let mut l = line.clone();
                    l.remove(line.len()-1);
                    l.remove(0);
                    let mut part : Part = HashMap::new();
                    for c in l.split(",") {
                        let t = c.split_once("=").expect("blouh");
                        part.insert(t.0.to_string(), t.1.parse::<i64>().expect("oh"));
                    }
                    parts.push(part);
                }

            }
        }
    }
    (workflows, parts)
}

fn is_part_accepted(workflows: &Workflows, part : &Part) -> bool {
    let mut wf_name = "in".to_string();

    while wf_name != "A" && wf_name != "R" {

        let wf = workflows.get(&wf_name).expect("blah");
        for instr in wf {
            let is_ok = match &instr.operand[..] {
                "*" => true,
                ">" => *part.get(&instr.variable).expect("al") > instr.value,
                "<" => *part.get(&instr.variable).expect("aia") < instr.value,
                _ => todo!("!!!"),
            };
            if is_ok {
                wf_name = instr.destination.clone();
                break;
            }
        }
    }
    wf_name == "A"
}


type PartP2 = HashMap<String, (i64,i64)>;

fn get_nb_accepted(workflows: &Workflows, parts: &PartP2, current_workflow: &str, current_inst: usize) -> i64 {

    if current_workflow == "A" {
        let mut i = 1;
        for (_,(min,max)) in parts {
            i *= (max-min)+1;
        }
        return i;
    } else if current_workflow == "R" {
        return 0;
    }

    let instr = &workflows.get(current_workflow).expect("a")[current_inst];
    match &instr.operand[..] {
        "*" => {
            get_nb_accepted(workflows, parts, &instr.destination, 0)
        },
        ">" => {
            let mut s = 0;
            let v = parts.get(&instr.variable).expect("bouh");
            if v.1 > instr.value {
                let mut parts_2 = parts.clone();
                parts_2.get_mut(&instr.variable).expect("brah").0 = instr.value+1;
                s += get_nb_accepted(workflows, &parts_2, &instr.destination, 0);
            }
            if v.0 <= instr.value {
                let mut parts_2 = parts.clone();
                parts_2.get_mut(&instr.variable).expect("brah").1 = instr.value;
                s += get_nb_accepted(workflows, &parts_2, current_workflow, current_inst+1);
            }
            s
        },
        "<" => {
            let mut s = 0;
            let v = parts.get(&instr.variable).expect("bouh");
            if v.0 < instr.value {
                let mut parts_2 = parts.clone();
                parts_2.get_mut(&instr.variable).expect("brah").1 = instr.value-1;
                s += get_nb_accepted(workflows, &parts_2, &instr.destination, 0);
            }
            if v.1 >= instr.value {
                let mut parts_2 = parts.clone();
                parts_2.get_mut(&instr.variable).expect("brah").0 = instr.value;
                s += get_nb_accepted(workflows, &parts_2, current_workflow, current_inst+1);
            }
            s
        },
        _ => todo!("???"),
    }
}

fn main() {

    let (workflows, parts) = parse_inputs();

    let mut p1 : i64 = 0;
    for part in parts {
        if is_part_accepted(&workflows, &part) {
            p1 += part.get("x").expect("x") + part.get("m").expect("m") + part.get("a").expect("a") + part.get("s").expect("s");
        }
    }
    println!("P1={}", p1);

    let parts_p2 = HashMap::from([
        ("x".to_string(), (1,4000)),
        ("m".to_string(), (1,4000)),
        ("a".to_string(), (1,4000)),
        ("s".to_string(), (1,4000)),
    ]);
    let p2 : i64 = get_nb_accepted(&workflows, &parts_p2, "in", 0);
    println!("P2={}", p2);
}
