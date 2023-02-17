import { ReactNode, useEffect, useState } from "react";
import Graph from "graphology";
import { SigmaContainer, useLoadGraph, useRegisterEvents, useSigma } from "@react-sigma/core";
import { useWorkerLayoutForceAtlas2 } from "@react-sigma/layout-forceatlas2";
import { useWorkerLayoutForce } from "@react-sigma/layout-force";
import "@react-sigma/core/lib/react-sigma.min.css";

const GraphEvents: React.FC = () => {
    // const [draggedNode, setDraggedNode] = useState<string | null>(null);
    const { start, kill } = useWorkerLayoutForceAtlas2({
        settings: {
            slowDown: 300,
            // barnesHutOptimize: false,
        }
        // settings: {
        // gravity: 0.00001,
        // gravity: 0.00001,
        // repulsion: 1,
        // attraction: 0.00005,
        // }
    });

    useEffect(() => {
        start();
        return () => {
            // stop();
            kill();
        };
    }, [start, kill]);

    return null;
};


function LoadGraph<T extends ReactNode>({ data }: { data: T[][] }) {
    const loadGraph = useLoadGraph();

    useEffect(() => {
        const graph = new Graph({ multi: false, allowSelfLoops: true, type: "directed" });

        let sideLength = Math.ceil(Math.sqrt(data.length));
        for (let u = 0; u < data.length; u++) {
            let x = u % sideLength;
            let y = Math.floor(u / sideLength);

            graph.addNode(u, {
                x: x * 10,
                y: y * 10,
                size: 5,
                label: `${u}`,
                color: "#eff6ff",
            });
        }

        for (let u = 0; u < data.length; u++) {
            for (let v of data[u]) {
                graph.addEdge(u, v, { color: "#bfdbfe" });
            }
        }

        loadGraph(graph);
    }, [loadGraph]);

    return null;
};

export default function GraphDisplay<T extends ReactNode>({ dataType, data }: { dataType: string, data: T[][] }): JSX.Element {
    return (
        <div>
            <span className="border-neutral-700 text-sm">{dataType} graph:</span>
            <div className="h-[30vh] bg-blue-50 border-blue-200">
                <SigmaContainer
                    settings={{ labelRenderedSizeThreshold: 7, labelDensity: 1000, }}
                    className="rounded border overflow-auto flex">
                    <GraphEvents />
                    <LoadGraph data={data} />
                </SigmaContainer>
            </div>
        </div>
    );
}
