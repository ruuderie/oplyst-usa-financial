import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuLabel from '../components/ui/context-menu/ContextMenuLabel.vue';

const meta = {
  title: 'ContextMenuLabel',
  component: ContextMenuLabel,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuLabel>;

export default meta;
type Story = StoryObj<typeof ContextMenuLabel>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};