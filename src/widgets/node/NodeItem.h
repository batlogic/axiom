#pragma once
#include <QtWidgets/QGraphicsObject>

namespace AxiomModel {
    class Node;
}

namespace AxiomGui {

    class SchematicCanvas;

    class NodeItem : public QObject, public QGraphicsItemGroup {
        Q_OBJECT

    public:
        AxiomModel::Node *node;

        explicit NodeItem(AxiomModel::Node *node, SchematicCanvas *parent);

    public slots:
        void setPos(QPoint newPos);
        void setSize(QSize newSize);

        void remove();

    private slots:
        void resizerSetPos(QPointF newPos);
        void resizerSetSize(QSizeF newSize);

    signals:
        void resizerPosChanged(QPointF newPos);
        void resizerSizeChanged(QSizeF newSize);
    };

}
