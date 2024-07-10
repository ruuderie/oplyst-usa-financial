import type { Meta, StoryObj } from '@storybook/vue3';

import SheetFooter from '../components/ui/sheet/SheetFooter.vue';

const meta = {
  title: 'SheetFooter',
  component: SheetFooter,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SheetFooter>;

export default meta;
type Story = StoryObj<typeof SheetFooter>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};