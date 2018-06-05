#include "IONode.h"

#include "Surface.h"
#include "Runtime.h"

using namespace MaximRuntime;

IONode::IONode(Surface *surface, MaximCommon::ControlType type, bool isRead, bool isWrite) : Node(surface) {
    _control = std::make_unique<IOControl>(this, type, isRead, isWrite);
    _moduleClass = std::make_unique<GeneratableModuleClass>(surface->runtime()->ctx(), module(), "ionode");
    _moduleClass->complete();
    deploy();
}

GeneratableModuleClass *IONode::compile() {
    if (_needsCompile) {
        _needsCompile = false;
        deploy();
    }
    return _moduleClass.get();
}

void IONode::remove() {
    _control->remove();
    Node::remove();
}

void IONode::setName(const std::string &name) {
    _name = name;
}

void IONode::fiddle() {
    auto rootSurface = dynamic_cast<RootSurface *>(surface());
    assert(rootSurface);
    rootSurface->nodeFiddled(this);
}

std::vector<Control*> IONode::controls() const {
    return std::vector<Control*> {_control.get()};
}

MaximCodegen::ModuleClass *IONode::moduleClass() {
    return _moduleClass.get();
}
