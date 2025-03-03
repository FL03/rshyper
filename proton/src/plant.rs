/*
    Appellation: plant <module>
    Contrib: @FL03
*/
use crate::models::WolframUTM;
use crate::nrt::{LPR, Triad, TriadClass};
use rstm::{Direction, State, Tail};

/// An isolated computational entity represented topologically in the Tonnetz using a triad;
/// the triad is used to describe the _headspace_ of the UTM, representing all possible
/// combinations of (state, symbol) pairs
#[derive(Clone, Debug, PartialEq)]
pub struct Plant {
    pub(crate) modulus: usize,
    pub(crate) triad: Triad,
    pub(crate) utm: WolframUTM,
}

impl Plant {
    pub fn new(triad: Triad) -> Self {
        let utm = WolframUTM::new(triad.notes, State(0));
        Plant {
            modulus: 0,
            triad,
            utm,
        }
    }
    /// returns the alphabet of the plant's headspace
    #[inline]
    pub const fn alphabet(&self) -> &[usize; 3] {
        &self.triad.notes
    }
    /// returns the class of the plant's headspace
    #[inline]
    pub fn class(&self) -> TriadClass {
        self.triad.class()
    }
    /// returns a copy of the plant's current head
    pub fn head(&self) -> rstm::Head<usize, usize> {
        self.utm().head().copied()
    }
    /// returns a copy to the plant's current modulus; the modulus is used to identify the
    /// _layer_ it is associated with
    #[inline]
    pub fn modulus(&self) -> usize {
        self.modulus
    }
    /// returns the current position of the plant's UTM
    #[inline]
    pub fn position(&self) -> usize {
        self.utm().position()
    }
    /// returns a copy of the plant's current state
    #[inline]
    pub fn state(&self) -> State<usize> {
        self.utm().state().copied()
    }
    /// returns a mutable reference to the plant's state
    #[inline]
    pub fn state_mut(&mut self) -> State<&mut usize> {
        self.utm_mut().state_mut()
    }
    /// returns an immutable reference to the plant's tape
    #[inline]
    pub fn tape(&self) -> &Vec<usize> {
        self.utm().tape()
    }
    /// returns a mutable reference to the plant's tape
    #[inline]
    pub fn tape_mut(&mut self) -> &mut Vec<usize> {
        self.utm_mut().tape_mut()
    }
    #[inline]
    /// returns an immutable reference to the plant's headspace; aka the triad.
    pub const fn triad(&self) -> &Triad {
        &self.triad
    }
    #[inline]
    /// returns a mutable reference to the plant's headspace
    pub fn triad_mut(&mut self) -> &mut Triad {
        &mut self.triad
    }
    /// returns an immutable reference to the engine of the plant; here, the engine is defined
    /// to be a Wolfram [2, 3] UTM
    pub fn utm(&self) -> &WolframUTM {
        &self.utm
    }
    /// returns a mutable reference to the plant's UTM
    pub fn utm_mut(&mut self) -> &mut WolframUTM {
        &mut self.utm
    }
    /// set the modulus of the plant
    pub fn set_modulus(&mut self, modulus: usize) {
        self.modulus = modulus;
    }
    /// set the triad of the plant
    pub fn set_triad(&mut self, triad: Triad) {
        self.triad = triad;
    }
    /// set the ruleset of the machine
    pub fn set_ruleset(&mut self, ruleset: crate::RuleSet<usize, usize>) {
        self.utm.set_ruleset(ruleset);
    }
    /// set the utm of the plant
    pub fn set_utm(&mut self, utm: WolframUTM) {
        self.utm = utm;
    }
    /// consumes this instance to create another with the given class
    pub fn with_class(self, class: TriadClass) -> Self {
        Plant {
            triad: Triad::new(self.triad.notes, class),
            ..self
        }
    }
    /// consumes this instance to create another with the given modulus
    pub fn with_modulus(self, modulus: usize) -> Self {
        Plant { modulus, ..self }
    }
    /// consumes this instance to create another with the given ruleset
    pub fn with_ruleset(self, ruleset: crate::RuleSet<usize, usize>) -> Self {
        Plant {
            utm: self.utm.with_ruleset(ruleset),
            ..self
        }
    }
    /// consumes this instance to create another with the given triad
    pub fn with_triad(self, triad: Triad) -> Self {
        Plant { triad, ..self }
    }
    /// consumes this instance to create another with the given utm
    pub fn with_utm(self, utm: WolframUTM) -> Self {
        Plant { utm, ..self }
    }
    /// checks if the given symbol is currently considered by the plant's headspace
    pub fn contains<Q>(&self, pitch: &Q) -> bool
    where
        Q: core::borrow::Borrow<usize>,
    {
        self.triad.contains(pitch)
    }
    ///
    pub fn is_valid(&self) -> bool {
        self.triad().is_valid()
    }
    /// applies the given transformation to the plant's headspace, modifying the utm accordingly
    pub fn transform(&self, transform: LPR) -> Self {
        // transform the triad
        let triad = self.triad().transform(transform);
        // update the utm's alphabet
        let utm = self.utm.clone().with_alphabet(triad.notes);
        Plant {
            modulus: self.modulus,
            triad,
            utm,
        }
    }
    /// mutably applies the given transformation to the plant's headspace, modifying the utm
    pub fn transform_inplace(&mut self, transform: LPR) {
        let triad = self.triad().transform(transform);
        self.utm_mut().set_alphabet(*triad.notes());
        self.set_triad(triad);
    }
    /// execute a single step of the plant's UTM, returning true if the step was successful
    pub fn step(&mut self) -> bool {
        let position = self.position();
        let head = self.head();

        // verify that the current symbol is within the plant's headspace
        if !self.contains(&head.symbol) {
            // todo: implement a more sophisticated motion planning algorithm
            for i in [LPR::Leading, LPR::Parallel, LPR::Relative] {
                if i.apply(&self.triad).contains(&head.symbol) {
                    self.transform_inplace(i);
                    break;
                }
            }
        }
        // Find the transition rule for the current state and symbol
        if let Some(&tail) = self.utm().ruleset().get(&head) {
            let Tail {
                state: new_state,
                symbol: new_symbol,
                direction,
            } = tail;
            // Update the tape
            if position >= self.utm.tape.len() {
                self.utm.tape.resize(position + 1, 0);
            }
            self.utm.tape[position] = new_symbol;

            // Update state
            self.utm.set_state(new_state);

            // Move the head
            match direction {
                Direction::Left => {
                    if position > 0 {
                        self.utm.position -= 1;
                    } else {
                        self.tape_mut().insert(0, 0);
                    }
                }
                Direction::Right => {
                    self.utm.position += 1;
                    if self.position() >= self.tape().len() {
                        self.tape_mut().push(0);
                    }
                }
                Direction::Stay => {}
            }

            true // Step successful
        } else {
            // No rule found - machine halts
            false
        }
    }

    /// execute the given program on the plant, returning the history of states and tapes
    pub fn run(
        &mut self,
        input: Vec<usize>,
    ) -> crate::Result<Vec<(Triad, State<usize>, Vec<usize>)>> {
        // Initialize the tape with the input
        self.utm.tape = input;
        self.utm.position = 0;

        let mut history = Vec::new();

        // Record initial state
        history.push((
            *self.triad(),
            self.state(),
            self.tape().clone(),
        ));

        // Run until the machine halts (i.e., step() returns false)
        while self.step() {
            // Record each state after a step
            history.push((
                *self.triad(),
                self.state(),
                self.tape().clone(),
            ));

            // prevent infinite loops
            if history.len() > usize::MAX {
                return Err(crate::Error::InfiniteLoop);
            }
        }

        Ok(history)
    }
}
