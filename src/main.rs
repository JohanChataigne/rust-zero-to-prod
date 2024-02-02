use std::collections::{HashMap, HashSet};

fn main() {
    let mut my_company = Company {
        departments: HashMap::new(),
    };

    println!("All employees: {:?}", my_company.list_all_employees());

    let it_department = String::from("IT");
    let hr_department = String::from("HR");
    let legal_department = String::from("Legal");
    let direction_department = String::from("Direction");

    let johan = String::from("Johan Chataigner");
    let camille = String::from("Camille Guiraud");

    my_company.add_employee_to_department(&johan, &it_department);
    my_company.add_employee_to_department(&johan, &it_department);

    my_company.add_employee_to_department(&camille, &hr_department);
    my_company.add_employee_to_department(&camille, &direction_department);

    println!(
        "IT employees: {:?}",
        my_company.list_department_employees(&it_department)
    );
    println!(
        "HR employees: {:?}",
        my_company.list_department_employees(&hr_department)
    );
    println!(
        "Legal employees: {:?}",
        my_company.list_department_employees(&legal_department)
    );
    println!(
        "Direction employees: {:?}",
        my_company.list_department_employees(&direction_department)
    );
    println!("All employees: {:?}", my_company.list_all_employees());
}

struct Company {
    departments: HashMap<String, HashSet<String>>,
}

impl Company {
    fn add_employee_to_department(&mut self, employee: &String, department: &String) {
        let current_employees = self
            .departments
            .entry(String::from(department))
            .or_default();
        current_employees.insert(String::from(employee));
    }

    fn list_department_employees(&self, department: &String) -> HashSet<String> {
        let mut employees = HashSet::new();
        match self.departments.get(department) {
            None => employees,
            Some(value) => {
                employees.clone_from(value);
                employees
            }
        }
    }

    fn list_all_employees(&self) -> HashSet<String> {
        let mut all_employees: HashSet<String> = HashSet::new();

        self.departments.keys().for_each(|department| {
            all_employees.extend(self.list_department_employees(department))
        });
        all_employees
    }
}
