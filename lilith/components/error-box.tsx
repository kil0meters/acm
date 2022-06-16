type ErrorBoxProps = {
  children: React.ReactNode;
};

export default function ErrorBox({ children }: ErrorBoxProps): JSX.Element {
  return (
    <div className="bg-red-500 ring-2 ring-red-700 text-red-50 rounded-md p-3">
      <h2 className="text-2xl font-bold">error.</h2>

      {children}
    </div>
  );
}
