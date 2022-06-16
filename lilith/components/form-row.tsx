type FormRowProps = {
  title: string;
  pattern?: string;
  placeholder: string;
  maxLength: number;
  inputType?: string;
};

export default function FormRow({
  title,
  pattern,
  placeholder,
  maxLength,
  inputType,
}: FormRowProps): JSX.Element {
  return (
    <div className="flex flex-col gap-2">
      <label htmlFor={title}>{title}</label>
      <input
        name={title}
        placeholder={placeholder}
        className="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
        pattern={pattern}
        minLength={1}
        maxLength={maxLength}
        type={inputType}
        required={true}
      />
    </div>
  );
}
