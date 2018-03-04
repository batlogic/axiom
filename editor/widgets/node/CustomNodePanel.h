#pragma once

#include <QtWidgets/QGraphicsObject>
#include <QtWidgets/QTextEdit>

#include "compiler/runtime/ErrorLog.h"

class QGraphicsProxyWidget;

namespace AxiomModel {

    class CustomNode;

}

namespace AxiomGui {

    class CustomNodePanel : public QGraphicsObject {
    Q_OBJECT

    public:

        AxiomModel::CustomNode *node;

        explicit CustomNodePanel(AxiomModel::CustomNode *node);

        QRectF boundingRect() const override;

        void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget) override;

    private slots:

        void updateSize();

        void setOpen(bool open);

        void setError(const MaximRuntime::ErrorLog &log);

        void clearError();

        void triggerUpdate();

        void triggerGeometryChange();

        void resizerChanged(QPointF topLeft, QPointF bottomRight);

    signals:

        void resizerSizeChanged(QSizeF newSize);

    private:

        QGraphicsProxyWidget *textProxy;
        QTextEdit *textEditor;
        bool hasErrors = false;
        bool showingErrors = false;

        bool eventFilter(QObject *object, QEvent *event) override;

        void controlTextChanged();

        static void moveCursor(QTextCursor &cursor, SourcePos pos, QTextCursor::MoveMode mode);

    };

}