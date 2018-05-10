#pragma once

#include <QtCore/QObject>
#include <QtWidgets/QGraphicsPathItem>
#include <deque>

#include "editor/model/Hookable.h"

namespace AxiomModel {
    class ConnectionWire;
}

namespace AxiomGui {

    class WireItem : public QObject, public QGraphicsPathItem, public AxiomModel::Hookable {
    Q_OBJECT

    public:
        AxiomModel::ConnectionWire *wire;

        explicit WireItem(QObject *parent, AxiomModel::ConnectionWire *wire);

    private slots:

        void updateRoute(const std::deque<QPoint> &route);

        void setIsActive(bool active);

        void remove();

    };

}
