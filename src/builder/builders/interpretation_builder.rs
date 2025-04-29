use phenopackets::schema::v2::core::{interpretation::ProgressStatus, Diagnosis, Interpretation};



pub struct InterpretationBuilder {
    id: String,
    progress_status: ProgressStatus,
    diagnosis: Option<Diagnosis>,
    summary: String
}

impl InterpretationBuilder {
    pub fn new(identifier: impl Into<String>) -> Self {
        Self { 
            id: identifier.into(), 
            progress_status: ProgressStatus::UnknownProgress, 
            diagnosis: None, 
            summary: String::default()
        }
    }

    pub fn summary(mut self, text: impl Into<String>) -> Self {
        self.summary = text.into();
        self
    }

    pub fn in_progress(mut self) -> Self {
        self.progress_status = ProgressStatus::InProgress;
        self
    }

    pub fn completed(mut self, diagnosis: Diagnosis) -> Self {
        self.progress_status = ProgressStatus::Completed;
        self.diagnosis = Some(diagnosis);
        self
    }

    pub fn solved(mut self, diagnosis: Diagnosis) -> Self {
        self.progress_status = ProgressStatus::Solved;
        self.diagnosis = Some(diagnosis);
        self
    }

    pub fn unsolved(mut self, diagnosis: Diagnosis) -> Self {
        self.progress_status = ProgressStatus::Unsolved;
        self.diagnosis = Some(diagnosis);
        self
    }

    pub fn build(self) -> Interpretation {
        Interpretation { 
            id:self.id, 
            progress_status: self.progress_status.into(), 
            diagnosis: self.diagnosis, 
            summary: self.summary 
        }
    }


}

