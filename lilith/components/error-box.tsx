type ErrorBoxProps = {
    children: string;
};

export default function ErrorBox({ children }: ErrorBoxProps): JSX.Element {
    return (
        <div className="bg-red-500 text-red-50 p-4 my-4 flex flex-col gap-2 rounded-md border-red-600 dark:border-red-500 dark:bg-red-700 border">
            <h1 className="text-2xl font-bold">Error</h1>

            <pre className="bg-red-700 dark:bg-red-800 overflow-auto p-2 rounded">
                <code>{children}</code>
            </pre>
        </div>
    );
}
