/*
    Appellation: wolfram <module>
    Contrib: @FL03
*/
use crate::{RuleSet, Symbolic};

use rstm::{Direction, Head, State, Tail};

/// An implementation of the Wolfram [2, 3] UTM that consideres two states and three symbols
#[derive(Clone, Debug, PartialEq)]
pub struct WolframUTM<Q = usize, S = usize>
where
    Q: Eq + core::hash::Hash,
    S: Symbolic,
{
    // the alphabet of the machine
    pub(crate) alphabet: [S; 3],
    // current position of the head of the machine
    pub(crate) position: usize,
    // a set of rules mapping one head to another using some shift
    pub(crate) ruleset: RuleSet<Q, S>,
    // the current state of the machine
    pub(crate) state: State<Q>,
    // the tape, or memory, of the machine
    pub(crate) tape: Vec<S>,
}

impl<Q, S> WolframUTM<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Symbolic,
{
    // Create a new UTM with a specific program
    pub fn new(alphabet: [S; 3], State(state): State<Q>) -> Self {
        WolframUTM {
            alphabet,
            position: 0,
            state: State(state),
            tape: Vec::new(),
            ruleset: RuleSet::new(),
        }
    }
    /// create a new instance with the given alphabet and default state
    pub fn from_alphabet(alphabet: [S; 3]) -> Self
    where
        Q: Default,
    {
        Self {
            alphabet,
            position: 0,
            state: State::default(),
            tape: Vec::new(),
            ruleset: RuleSet::new(),
        }
    }
    /// returns the alphabet of the machine
    pub const fn alphabet(&self) -> &[S; 3] {
        &self.alphabet
    }
    /// returns a copy of the machines current position
    #[inline]
    pub fn position(&self) -> usize {
        self.position
    }
    /// returns a reference to the machine's ruleset
    pub const fn ruleset(&self) -> &RuleSet<Q, S> {
        &self.ruleset
    }
    /// returns a mutable reference to the machine's ruleset
    pub fn ruleset_mut(&mut self) -> &mut RuleSet<Q, S> {
        &mut self.ruleset
    }
    /// returns a copy of the machine's current state
    #[inline]
    pub fn state(&self) -> State<&Q> {
        self.state.to_ref()
    }
    /// returns a mutable reference to the machine's state
    pub fn state_mut(&mut self) -> State<&mut Q> {
        self.state.to_mut()
    }
    /// returns an immutable reference to the tape
    pub const fn tape(&self) -> &Vec<S> {
        &self.tape
    }
    /// returns a mutable reference to the tape
    pub fn tape_mut(&mut self) -> &mut Vec<S> {
        &mut self.tape
    }
    /// returns the head of the machine
    pub fn head(&self) -> Head<&Q, &S> {
        // self.get_current_symbol().cloned().unwrap_or_default()
        Head::new(self.state(), &self.tape[self.position])
    }
    /// check if a symbol is in the alphabet
    pub fn contains(&self, symbol: &S) -> bool {
        self.alphabet.contains(symbol)
    }
    /// get the current symbol on the tape
    pub fn get_current_symbol(&self) -> Option<&S> {
        self.tape.get(self.position)
    }
    /// get the tail for the current head
    pub fn get_tail(&self) -> Option<&Tail<Q, S>>
    where
        Q: Clone,
    {
        self.ruleset.get(&self.head().cloned())
    }
    /// get the tail for a given head
    pub fn get_tail_for(&self, head: Head<Q, S>) -> &Tail<Q, S> {
        &self.ruleset[&head]
    }
    /// process some input
    pub fn run(&mut self, input: Vec<S>) -> crate::Result<Vec<(State<Q>, [S; 3], Vec<S>)>>
    where
        Q: Clone,
    {
        // Initialize the tape with the input
        self.tape = input;
        self.position = 0;

        let mut history = vec![(
            self.state().cloned(),
            self.alphabet().clone(),
            self.tape().clone(),
        )];

        // Run until the machine halts (i.e., step() returns false)
        while self.step() {
            // Record each state after a step
            history.push((
                self.state().cloned(),
                self.alphabet().clone(),
                self.tape().clone(),
            ));

            // Optional: Add a safety limit to prevent infinite loops in development
            if history.len() > usize::MAX {
                return Err(crate::Error::InfiniteLoop);
            }
        }

        Ok(history)
    }
    /// execute one step of the machine
    pub fn step(&mut self) -> bool
    where
        Q: Clone,
    {
        let symbol = self.get_current_symbol().cloned().unwrap_or_default();

        // Check if the current symbol is in our current triad context
        if !self.contains(&symbol) {
            panic!("symbol {} not in any accessible triad context", symbol);
        }
        let head = Head::new(self.state.clone(), symbol);
        // Find the transition rule for the current state and symbol
        if let Some(tail) = self.ruleset.get(&head) {
            let Tail {
                state: new_state,
                symbol: new_symbol,
                direction,
            } = tail.clone();
            // Update the tape
            if self.position >= self.tape.len() {
                self.tape.resize(self.position + 1, S::default());
            }
            self.tape[self.position] = new_symbol;

            // Update state
            self.state = new_state;

            // Move the head
            match direction {
                Direction::Left => {
                    if self.position > 0 {
                        self.position -= 1;
                    } else {
                        self.tape.insert(0, S::default());
                    }
                }
                Direction::Right => {
                    self.position += 1;
                    if self.position >= self.tape.len() {
                        self.tape.push(S::default());
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
    /// set the alphabet of the machine
    pub fn set_alphabet(&mut self, alphabet: [S; 3]) {
        self.alphabet = alphabet;
    }
    /// set the ruleset of the machine
    pub fn set_ruleset(&mut self, ruleset: RuleSet<Q, S>) {
        self.ruleset = ruleset;
    }
    /// set the state of the machine
    pub fn set_state(&mut self, State(state): State<Q>) {
        self.state = State(state);
    }
    /// consumes this instance to create another with the given alphabet
    pub fn with_alphabet(self, alphabet: [S; 3]) -> Self {
        Self { alphabet, ..self }
    }
    /// consumes this instance to create another with the given ruleset
    pub fn with_ruleset(self, ruleset: RuleSet<Q, S>) -> Self {
        Self { ruleset, ..self }
    }
    /// consumes this instance to create another with the given state
    pub fn with_state(self, State(state): State<Q>) -> Self {
        Self {
            state: State(state),
            ..self
        }
    }
}

impl WolframUTM {
    /// returns a string representation of the machine
    pub fn pretty_print(&self) -> String {
        let mut result = String::new();

        // Print the tape with the current position marked
        for (i, &symbol) in self.tape.iter().enumerate() {
            if i == self.position {
                result.push_str(&format!("[{}]", symbol));
            } else {
                result.push_str(&format!(" {} ", symbol));
            }
        }

        // Add state and triad information
        result.push_str(&format!(
            "(State: {:?}, Alphabet: {:?})",
            self.state, self.alphabet
        ));

        result
    }
}

impl<Q, S> core::fmt::Display for WolframUTM<Q, S>
where
    Q: Eq + core::fmt::Debug + core::hash::Hash,
    S: Symbolic,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{head:?} @ {pos}: {tape:?}",
            head = self.head(),
            pos = self.position(),
            tape = self.tape()
        )
    }
}

impl<Q, S> core::ops::Index<usize> for WolframUTM<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Symbolic,
{
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tape[index]
    }
}

impl<Q, S> core::ops::Index<Head<Q, S>> for WolframUTM<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Symbolic,
{
    type Output = Tail<Q, S>;

    fn index(&self, index: Head<Q, S>) -> &Self::Output {
        &self.ruleset[&index]
    }
}
