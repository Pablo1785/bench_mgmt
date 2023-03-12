use crate::{
    dao::project::ProjectDAO,
    domain::{assignment::Assignment, error::Error},
    service::{employee::EmployeeService, project::ProjectService},
};

pub struct ProjectServiceImpl<P, E>
where
    P: ProjectDAO,
    E: EmployeeService,
{
    project_dao: P,
    employee_service: E,
}

impl<P, E> ProjectService for ProjectServiceImpl<P, E>
where
    P: ProjectDAO,
    E: EmployeeService,
{
    fn find_matching_employees(
        &self,
        project: &crate::domain::project::Project,
    ) -> std::collections::HashMap<
        crate::domain::position::Position,
        std::collections::HashSet<crate::domain::employee::Employee>,
    > {
        todo!()
    }

    fn try_assign(
        &self,
        employee: crate::domain::employee::Employee,
        position: crate::domain::position::Position,
        project: crate::domain::project::Project,
    ) -> Result<crate::domain::assignment::Assignment, crate::domain::error::Error> {
        match (
            self.employee_service
                .is_employee_matching_position(&employee, &position),
            self.employee_service
                .is_employee_matching_project(&employee, &project),
        ) {
            (true, true) => Ok(Assignment::new(employee, position, project)),
            (false, true) => Err(Error::EmployeeDoesNotMatchPositionError { employee, position }),
            (true, false) => Err(Error::EmployeeDoesNotMatchProjectError { employee, project }),
            (false, false) => Err(Error::EmployeeDoesNotMatchProjectNorPositionError {
                employee,
                project,
                position,
            }),
        }
    }

    fn try_save(
        &mut self,
        project: crate::domain::project::Project,
    ) -> Result<(), crate::domain::error::Error> {
        todo!()
    }

    fn find_matching_projects(
        &self,
        employee: &crate::domain::employee::Employee,
    ) -> std::collections::HashSet<crate::domain::project::Project> {
        self.project_dao
            .get_all()
            .into_iter()
            .filter(|project| {
                self.employee_service
                    .is_employee_matching_project(employee, project)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dao::project::MockProjectDAO,
        domain::{employee::Employee, position::Position, project::Project},
        service::employee::MockEmployeeService,
    };

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_try_assign_correct() {
        let mut mock_employee_service = MockEmployeeService::new();
        mock_employee_service
            .expect_is_employee_matching_project()
            .return_const(true);
        mock_employee_service
            .expect_is_employee_matching_position()
            .return_const(true);
        let project_service = ProjectServiceImpl::<MockProjectDAO, MockEmployeeService> {
            project_dao: MockProjectDAO::new(),
            employee_service: mock_employee_service,
        };
        let employee = Employee::default();
        let position = Position::default();
        let project = Project::default();

        let result = project_service.try_assign(employee, position, project);
        assert!(if let Ok(_) = result { true } else { false });
    }

    #[test]
    fn test_try_assign_non_matching_position() {
        let mut mock_employee_service = MockEmployeeService::new();
        mock_employee_service
            .expect_is_employee_matching_project()
            .return_const(true);
        mock_employee_service
            .expect_is_employee_matching_position()
            .return_const(false);
        let project_service = ProjectServiceImpl::<MockProjectDAO, MockEmployeeService> {
            project_dao: MockProjectDAO::new(),
            employee_service: mock_employee_service,
        };
        let employee = Employee::default();
        let position = Position::default();
        let project = Project::default();

        let result = project_service.try_assign(employee, position, project);
        assert!(
            if let Err(Error::EmployeeDoesNotMatchPositionError { employee, position }) = result {
                true
            } else {
                false
            }
        );
    }

    #[test]
    fn test_try_assign_non_matching_project() {
        let mut mock_employee_service = MockEmployeeService::new();
        mock_employee_service
            .expect_is_employee_matching_project()
            .return_const(false);
        mock_employee_service
            .expect_is_employee_matching_position()
            .return_const(true);
        let project_service = ProjectServiceImpl::<MockProjectDAO, MockEmployeeService> {
            project_dao: MockProjectDAO::new(),
            employee_service: mock_employee_service,
        };
        let employee = Employee::default();
        let position = Position::default();
        let project = Project::default();

        let result = project_service.try_assign(employee, position, project);
        assert!(
            if let Err(Error::EmployeeDoesNotMatchProjectError { employee, project }) = result {
                true
            } else {
                false
            }
        );
    }

    #[test]
    fn test_try_assign_non_matching_position_nor_project() {
        let mut mock_employee_service = MockEmployeeService::new();
        mock_employee_service
            .expect_is_employee_matching_project()
            .return_const(false);
        mock_employee_service
            .expect_is_employee_matching_position()
            .return_const(false);
        let project_service = ProjectServiceImpl::<MockProjectDAO, MockEmployeeService> {
            project_dao: MockProjectDAO::new(),
            employee_service: mock_employee_service,
        };
        let employee = Employee::default();
        let position = Position::default();
        let project = Project::default();

        let result = project_service.try_assign(employee, position, project);
        assert!(
            if let Err(Error::EmployeeDoesNotMatchProjectNorPositionError {
                employee,
                project,
                position,
            }) = result
            {
                true
            } else {
                false
            }
        );
    }
}
