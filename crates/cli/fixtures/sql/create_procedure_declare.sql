CREATE PROCEDURE remove_emp (employee_id NUMBER) AS
   µdeclare
   BEGIN
      DELETE FROM employees
      WHERE employees.employee_id = remove_emp.employee_id;
   tot_emps := tot_emps - 1;
   END;
