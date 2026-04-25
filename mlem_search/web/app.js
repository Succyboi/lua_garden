//#region Feature

let audio = document.getElementById('release-audio');

function featurePlayAudio(feature) {
    audio.src = encodeURI("/" + feature.path);
    audio.load();
    audio.play();
}

//#endregion
//#region Graph

let SelectedNodes = new Set();
let Features = new Map();

const InitGraphData = {
      nodes: [],
      links: [],
};
let Graph;
const NumRandomFeatures = 1;
const NumRandomFeatureSimilarities = 2;
const NumSimilarFeatures = 16;

const DistanceMultiplier = 8 * 100000;
const DistanceMin = 24;
const DistanceMax = 64;

const ColorDistanceMultiplier = 4 * 10000;
const ColorAliveLightness = 50;
const ColorDeadLightness = 80;

async function graphInit() {
    Graph = new ForceGraph(document.getElementById('graph'))
        .graphData(InitGraphData)
        .nodeRelSize(8)
        .cooldownTime(500)
        .nodeColor(node => graphNodeColor(node))
        .nodeLabel(node => {
            let feature = Features.get(node.id);
            
            return `<b>${feature.id}</b><br>${feature.path}`;
        })
        .onNodeClick((node, event) => {
            if (event.ctrlKey || event.shiftKey || event.altKey) { // multi-selection
            SelectedNodes.has(node) ? SelectedNodes.delete(node) : SelectedNodes.add(node);
            } else { // single-selection
            const untoggle = SelectedNodes.has(node) && SelectedNodes.size === 1;
            SelectedNodes.clear();
            !untoggle && SelectedNodes.add(node);
            }

            Graph.nodeColor(Graph.nodeColor()); // update color of selected nodes
            Graph.centerAt(node.x, node.y, 250);
            
            let feature = Features.get(node.id);
            graphAddSimilar(feature);
            featurePlayAudio(feature);
        })
        .onNodeDrag((node, translate) => {
            if (SelectedNodes.has(node)) { // moving a selected node
            [...SelectedNodes]
                .filter(selNode => selNode !== node) // don't touch node being dragged
                .forEach(node => ['x', 'y'].forEach(coord => node[`f${coord}`] = node[coord] + translate[coord])); // translate other nodes by same amount
            }
        })
        .onNodeDragEnd(node => {
            if (SelectedNodes.has(node)) { // finished moving a selected node
            [...SelectedNodes]
                .filter(selNode => selNode !== node) // don't touch node being dragged
                .forEach(node => ['x', 'y'].forEach(coord => node[`f${coord}`] = undefined)); // unfix controlled nodes
            }
        })
        .onNodeHover(node => {
            if (node == undefined) { return; }
            
            let feature = Features.get(node.id);
            featurePlayAudio(feature);
        });

    for (let i = 0; i < NumRandomFeatures; i++) {
        await graphAddRandom();
    }

    for (let i = 0; i < NumRandomFeatureSimilarities; i++) {
        let featureKeys = Array.from(Features.keys());
        let feature = Features.get(featureKeys[Math.floor(Math.random() * featureKeys.length)]);
        await graphAddSimilar(feature);
    }
}

function graphNodeColor(node) {
    let feature = Features.get(node.id);
    let hue = Math.floor(feature.hue) % 360;
    let saturation = 25;
    let lightness = feature.dead ? ColorDeadLightness : ColorAliveLightness;
    let baseColor = `hsl(${hue}, ${saturation}%, ${lightness}%)`;

    return SelectedNodes.has(node) ? 'darkorange' : baseColor;
}

function graphAddFeature(feature, fromFeature, distance) {
    if (Features.has(feature.id)) { return; }

    feature.hue = fromFeature == undefined ? (Math.random() * 361) : (fromFeature.hue + distance * ColorDistanceMultiplier);
    feature.dead = false;
    Features.set(feature.id, feature);

    const { nodes, links } = Graph.graphData();
    let id = feature.id;
    let node = { id }
    
    Graph.graphData({
        nodes: [...nodes, node],
        links: [...links]
    });
}

function graphConnectFeature(sourceFeature, targetFeature, distance) {
    const { nodes, links } = Graph.graphData();
    let sourceId = sourceFeature.id;
    let targetId = targetFeature.id;

    if (links.some((l) => l.source.id == sourceId && l.target.id == targetId)) { return; }

    let connection = { source: sourceId, target: targetId, distance: distance };
    Graph.graphData({
        nodes: [...nodes],
        links: [...links, connection]
    });
    Graph.d3Force("link").distance(link => Math.max(Math.min(link.distance * DistanceMultiplier, DistanceMax), DistanceMin));
}

async function graphAddRandom() {
    const endPointRandom = "/random";

    await fetch(endPointRandom)
        .then(response => {
            if (!response.ok) {
                throw new Error(`Couldn't get ${endPointRandom}`);
            }
            return response.json();
        })
        .then(data => {
            graphAddFeature(data.feature);
            graphAddSimilar(data.feature);
        })
        .catch(error => {
            console.error('Error:', error);
        });
}

async function graphAddSimilar(sourceFeature) {
    let id = sourceFeature.id;
    let count = NumSimilarFeatures;
    let endPointSimilar = `/similar/${id}-${count}`;

    if (sourceFeature.dead) { return; }
    sourceFeature.dead = true;
    Features.set(sourceFeature.id, sourceFeature);

    await fetch(endPointSimilar)
        .then(response => {
            if (!response.ok) {
                throw new Error(`Couldn't get ${endPointSimilar}`);
            }
            return response.json();
        })
        .then(data => {
            for (let i = 0; i < data.features.length; i++) {
                let distance = data.features[i].distance;
                let targetFeature = data.features[i].feature;
                
                graphAddFeature(targetFeature, sourceFeature, distance);
                graphConnectFeature(sourceFeature, targetFeature, distance);
            }
        })
        .catch(error => {
            console.error('Error:', error);
        });
}

//#endregion
//#region Init

graphInit();

//#endregion