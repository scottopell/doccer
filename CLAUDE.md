<project_goal>
emit human-readable text for terminal viewing of rust documentation
</project_goal>


<commands>
    <test> cargo test </test>
</commands>

<code_instructions>
    <comments>All comments should be written in context of the project as a
    declaration of non-intuitive parts of the code. Comments should never refer
    to the current implementation task.
        <bad_example>Indent to account for the expected format for trailing curly braces</bad_example>
        <good_example>The indentation at this level is decremented as we close a
        syntactic construct</good_example>

        Comments should be sparing and concise when used.
    </comments>
    
    <CRITICAL_RULES>
        NEVER hardcode test-specific behaviors into the main codebase. This includes:
        - Special handling for specific crate, module, struct, or function names
        - Conditional logic based on specific function or type names
        - Fixed output formats for specific test fixtures
        - Custom indentation rules for specific trait implementations
        
        The core implementation (src/main.rs) must be able to handle ANY Rust crate,
        not just our test fixtures. Hard-coding violates this principle and undermines
        the generality of the solution.
        
        Instead, use unit tests to verify that the generic implementation works
        correctly across all required use cases. If tests are failing because the
        implementation doesn't match specific expectations, either:
        1. Fix the general implementation to handle all cases correctly
        2. Update the test expectations to match the correct output
        
        This separation between code and tests is MANDATORY. Violating this rule
        will result in immediate rejection of any changes.
    </CRITICAL_RULES>
</code_instructions>

