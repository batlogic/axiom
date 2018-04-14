#include "GridItem.h"

#include <QtCore/QDataStream>
#include <iostream>

#include "GridSurface.h"

using namespace AxiomModel;

GridItem::GridItem(GridSurface *parent, QPoint pos, QSize size)
    : parentSurface(parent), m_pos(parent->grid.findNearestAvailable(pos, size)), m_size(size) {
    parentSurface->grid.setRect(m_pos, m_size, this);
    parentSurface->flushGrid();
}

bool GridItem::isDragAvailable(QPoint delta) {
    return parentSurface->grid.isRectAvailable(dragStartPos + delta, m_size, this);
}

void GridItem::setSize(QSize size) {
    if (!isResizable()) return;

    if (size != m_size) {
        if (size.width() < 1 || size.height() < 1 || !parentSurface->grid.isRectAvailable(m_pos, size, this)) return;

        emit beforeSizeChanged(size);
        parentSurface->grid.moveRect(m_pos, m_size, m_pos, size, this);
        parentSurface->flushGrid();
        m_size = size;
        emit sizeChanged(size);
    }
}

void GridItem::setCorners(QPoint topLeft, QPoint bottomRight) {
    if (!isResizable()) return;

    auto newSize = QSize(bottomRight.x() - topLeft.x(), bottomRight.y() - topLeft.y());
    if (newSize.width() < 1 || newSize.height() < 1) return;
    if (topLeft == m_pos && newSize == m_size) return;

    if (!parentSurface->grid.isRectAvailable(topLeft, newSize, this)) {
        // try resizing just horizontally or just vertically
        auto hTopLeft = QPoint(topLeft.x(), m_pos.y());
        auto hSize = QSize(newSize.width(), m_size.height());
        auto vTopLeft = QPoint(m_pos.x(), topLeft.y());
        auto vSize = QSize(m_size.width(), newSize.height());
        if (parentSurface->grid.isRectAvailable(hTopLeft, hSize, this)) {
            topLeft = hTopLeft;
            newSize = hSize;
        } else if (parentSurface->grid.isRectAvailable(vTopLeft, vSize, this)) {
            topLeft = vTopLeft;
            newSize = vSize;
        } else return;
    }

    if (topLeft == m_pos && newSize == m_size) return;
    parentSurface->grid.moveRect(m_pos, m_size, topLeft, newSize, this);
    parentSurface->flushGrid();
    emit beforePosChanged(topLeft);
    m_pos = topLeft;
    emit posChanged(m_pos);
    emit beforeSizeChanged(newSize);
    m_size = newSize;
    emit sizeChanged(m_size);
}

void GridItem::select(bool exclusive) {
    if (exclusive || !m_selected) {
        m_selected = true;
        emit selectedChanged(m_selected);
        emit selected(exclusive);
    }
}

void GridItem::deselect() {
    if (!m_selected) return;
    m_selected = false;
    emit selectedChanged(m_selected);
    emit deselected();
}

void GridItem::startDragging() {
    dragStartPos = m_pos;
}

void GridItem::dragTo(QPoint delta) {
    setPos(dragStartPos + delta, false, false);
}

void GridItem::finishDragging() {
}

void GridItem::remove() {
    emit removed();
    emit cleanup();
}

void GridItem::setPos(QPoint pos, bool updateGrid, bool checkPositions) {
    if (pos != m_pos) {
        if (checkPositions && !parentSurface->grid.isRectAvailable(pos, m_size, this)) return;

        if (pos == m_pos) return;

        emit beforePosChanged(pos);
        if (updateGrid) {
            parentSurface->grid.moveRect(m_pos, m_size, pos, m_size, this);
            parentSurface->flushGrid();
        }
        m_pos = pos;
        emit posChanged(pos);
    }
}

void GridItem::serialize(QDataStream &stream, QPoint offset) const {
    stream << m_selected;
    stream << (m_pos + offset);
    stream << m_size;
}

void GridItem::deserialize(QDataStream &stream, QPoint offset) {
    bool selected;
    stream >> selected;
    QPoint pos;
    stream >> pos;
    QSize size;
    stream >> size;

    setPos(pos + offset, true, false);
    setSize(size);
    if (selected) select(false);
}
