use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: T,
}

impl<T: Copy + PartialEq> InputCell<T> {
    fn new(value: T) -> Self {
        InputCell { value }
    }
}

struct ComputeCell<'a, T> {
    cell_id: ComputeCellID,
    dependencies: Vec<CellID>,
    compute_func: Box<'a + Fn(&[T]) -> T>,
    value: T,
}

impl<'a, T: Copy + PartialEq> ComputeCell<'a, T> {
    fn new<F: 'a + Fn(&[T]) -> T>(
        cell_id: ComputeCellID,
        dependencies: &[CellID],
        compute_func: F,
        value: T,
    ) -> Self {
        ComputeCell {
            cell_id,
            dependencies: dependencies.to_vec(),
            compute_func: Box::new(compute_func),
            value,
        }
    }
}

struct Callback<'a, T> {
    id: ComputeCellID,
    callback: Box<'a + FnMut(T) -> ()>,
}

impl<'a, T: Copy + PartialEq> Callback<'a, T> {
    fn new<F: 'a + FnMut(T) -> ()>(id: ComputeCellID, callback: F) -> Self {
        Callback {
            id,
            callback: Box::new(callback),
        }
    }
}

pub struct Reactor<'a, T> {
    input_cells: Vec<InputCell<T>>,
    compute_cells: Vec<ComputeCell<'a, T>>,
    callbacks: HashMap<usize, Callback<'a, T>>,
    next_callback_id: CallbackID,
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: Vec::new(),
            compute_cells: Vec::new(),
            callbacks: HashMap::new(),
            next_callback_id: CallbackID(0),
        }
    }

    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.input_cells.push(InputCell::new(initial));

        InputCellID(self.input_cells.len() - 1)
    }

    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        if let Some(&cell_id) = dependencies.iter().find(|&&id| self.value(id).is_none()) {
            Err(cell_id)
        } else {
            let cell_id = ComputeCellID(self.compute_cells.len());
            let value = self.compute_value(dependencies, &compute_func);

            let compute_cell = ComputeCell::new(cell_id, dependencies, compute_func, value);
            self.compute_cells.push(compute_cell);

            Ok(cell_id)
        }
    }

    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(cell_id) => self.input_cells.get(cell_id.0).map(|cell| cell.value),
            CellID::Compute(cell_id) => self.compute_cells.get(cell_id.0).map(|cell| cell.value),
        }
    }

    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if self.input_cells.len() <= id.0 {
            return false;
        }

        self.input_cells[id.0].value = new_value;

        let mut value_history = HashMap::new();

        self.update_dependents(CellID::Input(id), &mut value_history);

        for (id, old_value) in value_history {
            let new_value = self.compute_cells[id].value;

            if old_value == new_value {
                continue;
            }

            for callback in self.callbacks
                .values_mut()
                .filter(|callback| callback.id.0 == id)
            {
                (callback.callback)(self.compute_cells[id].value);
            }
        }

        true
    }

    fn compute_value(&self, dependencies: &[CellID], compute_func: impl Fn(&[T]) -> T) -> T {
        let values = dependencies
            .iter()
            .map(|&id| self.value(id).unwrap())
            .collect::<Vec<_>>();

        compute_func(&values)
    }

    fn update_dependents(&mut self, id: CellID, value_history: &mut HashMap<usize, T>) {
        if let CellID::Compute(cell_id) = id {
            let (old_value, new_value) = {
                let cell = &self.compute_cells[cell_id.0];

                (
                    cell.value,
                    self.compute_value(&cell.dependencies, cell.compute_func.as_ref()),
                )
            };

            if old_value == new_value {
                return;
            }

            value_history.entry(cell_id.0).or_insert(old_value);
            self.compute_cells[cell_id.0].value = new_value;
        }

        let dependents = self.compute_cells
            .iter()
            .filter(|cell| cell.dependencies.contains(&id))
            .map(|cell| cell.cell_id)
            .collect::<Vec<_>>();

        for dependent in dependents {
            self.update_dependents(CellID::Compute(dependent), value_history);
        }
    }

    pub fn add_callback<F: 'a + FnMut(T) -> ()>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        if self.compute_cells.len() <= id.0 {
            return None;
        }

        let callback_id = self.next_callback_id;
        self.next_callback_id.0 += 1;

        self.callbacks
            .insert(callback_id.0, Callback::new(id, callback));

        Some(callback_id)
    }

    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        if self.compute_cells.len() <= cell.0 {
            Err(RemoveCallbackError::NonexistentCell)
        } else if self.callbacks.remove(&callback.0).is_some() {
            Ok(())
        } else {
            Err(RemoveCallbackError::NonexistentCallback)
        }
    }
}
