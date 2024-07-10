import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuCheckboxItem from '../components/ui/context-menu/ContextMenuCheckboxItem.vue';

const meta = {
  title: 'ContextMenuCheckboxItem',
  component: ContextMenuCheckboxItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuCheckboxItem>;

export default meta;
type Story = StoryObj<typeof ContextMenuCheckboxItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};