import type { Meta, StoryObj } from '@storybook/vue3';

import SheetHeader from '../components/ui/sheet/SheetHeader.vue';

const meta = {
  title: 'SheetHeader',
  component: SheetHeader,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SheetHeader>;

export default meta;
type Story = StoryObj<typeof SheetHeader>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};