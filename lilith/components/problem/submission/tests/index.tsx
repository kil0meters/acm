export const SUBMISSION_TESTS_QUERY = "submissions-tests";

export interface Test {
  id: number;
  index: number;
  input: string;
  expected_output: string;
}

export interface TestResult extends Test {
  output: string;
  success: boolean;
  runtime: number;
}
