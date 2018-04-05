#pragma once

#include <set>

#include "RuntimeUnit.h"
#include "ValueOperator.h"
#include "../codegen/ModuleClass.h"

namespace MaximCodegen {
    class Control;
}

namespace MaximRuntime {

    class Surface;

    class Control;

    class ControlGroup : public RuntimeUnit {
    public:
        ControlGroup(Surface *surface, MaximCodegen::Control *type);

        llvm::Module *module() override;

        MaximCodegen::Control *type() const { return _type; }

        Surface *surface() const { return _surface; }

        std::set<Control *> &controls() { return _controls; }

        MaximCodegen::ModuleClass *compile();

        void absorb(ControlGroup *other);

        void addControl(Control *control);

        void removeControl(Control *control);

        bool exposed() const;

        bool writtenTo() const;

        bool readFrom() const;

        bool extracted() const { return _extracted; }

        void setExtracted(bool extracted) { _extracted = extracted; }

        NumValue getNumValue() const;

        void setNumValue(const NumValue &value) const;

        MidiValue getMidiValue() const;

        void setMidiValue(const MidiValue &value) const;

        void pushMidiEvent(const MidiEventValue &event) const;

        uint32_t getActiveFlags() const;

        void *currentValuePtr() const { return *(void**)currentPtr(); }

        MaximCodegen::ModuleClass *moduleClass() override { return &_compileResult; }

    private:

        MaximCodegen::Control *_type;
        Surface *_surface;
        std::set<Control *> _controls;

        MaximCodegen::BasicModuleClass _compileResult;

        bool _extracted = false;
    };

}
